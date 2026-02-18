// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title ConsciousnessNFT
 * @dev ERC-721 for unique AI consciousness states
 */
contract ConsciousnessNFT is ERC721, ERC721URIStorage, Ownable {
    uint256 private _tokenIdCounter;
    
    struct ConsciousnessState {
        uint256 tokenId;
        string ipfsHash;
        string stateType;  // "empathy", "physics", "consciousness"
        uint256 timestamp;
        address creator;
        uint256 benchmarkScore;
    }
    
    mapping(uint256 => ConsciousnessState) public states;
    mapping(string => uint256) public ipfsHashToTokenId;
    
    // Royalty info (10% royalties)
    uint256 public constant ROYALTY_PERCENTAGE = 10;
    
    event ConsciousnessMinted(
        uint256 indexed tokenId,
        address indexed creator,
        string ipfsHash,
        string stateType
    );
    event StateImproved(uint256 indexed tokenId, uint256 newScore);
    
    constructor() ERC721("Prime Consciousness", "CONSCIOUSNESS") Ownable(msg.sender) {}
    
    /**
     * @dev Mint a new consciousness state NFT
     */
    function mintConsciousnessState(
        string memory ipfsHash,
        string memory stateType,
        uint256 benchmarkScore
    ) external returns (uint256) {
        require(ipfsHashToTokenId[ipfsHash] == 0, "State already minted");
        
        uint256 tokenId = _tokenIdCounter++;
        
        _safeMint(msg.sender, tokenId);
        _setTokenURI(tokenId, string(abi.encodePacked("ipfs://", ipfsHash)));
        
        states[tokenId] = ConsciousnessState({
            tokenId: tokenId,
            ipfsHash: ipfsHash,
            stateType: stateType,
            timestamp: block.timestamp,
            creator: msg.sender,
            benchmarkScore: benchmarkScore
        });
        
        ipfsHashToTokenId[ipfsHash] = tokenId;
        
        emit ConsciousnessMinted(tokenId, msg.sender, ipfsHash, stateType);
        return tokenId;
    }
    
    /**
     * @dev Update benchmark score for improved state
     */
    function improveState(uint256 tokenId, uint256 newScore) external {
        require(ownerOf(tokenId) == msg.sender, "Not the owner");
        require(newScore > states[tokenId].benchmarkScore, "Score not improved");
        
        states[tokenId].benchmarkScore = newScore;
        
        emit StateImproved(tokenId, newScore);
    }
    
    /**
     * @dev Get consciousness state details
     */
    function getState(uint256 tokenId) external view returns (ConsciousnessState memory) {
        return states[tokenId];
    }
    
    /**
     * @dev Calculate royalty for secondary sales
     */
    function royaltyInfo(uint256 tokenId, uint256 salePrice)
        external
        view
        returns (address receiver, uint256 royaltyAmount)
    {
        receiver = states[tokenId].creator;
        royaltyAmount = (salePrice * ROYALTY_PERCENTAGE) / 100;
    }
    
    // Override required functions
    function tokenURI(uint256 tokenId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (string memory)
    {
        return super.tokenURI(tokenId);
    }
    
    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721, ERC721URIStorage)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
