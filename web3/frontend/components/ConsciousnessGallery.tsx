import React, { useState, useEffect } from 'react';

export default function ConsciousnessGallery() {
  const [nfts, setNfts] = useState([]);
  const [filter, setFilter] = useState('all');

  useEffect(() => {
    // Mock NFT data - in production, fetch from blockchain
    setNfts([
      {
        id: 1,
        type: 'empathy',
        score: 0.92,
        ipfsHash: 'QmEmpathy1',
        timestamp: '2026-02-15',
      },
      {
        id: 2,
        type: 'physics',
        score: 0.95,
        ipfsHash: 'QmPhysics1',
        timestamp: '2026-02-14',
      },
      {
        id: 3,
        type: 'consciousness',
        score: 0.88,
        ipfsHash: 'QmConsc1',
        timestamp: '2026-02-13',
      },
    ]);
  }, []);

  const filteredNfts = filter === 'all' 
    ? nfts 
    : nfts.filter(nft => nft.type === filter);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-bold">Consciousness NFT Gallery</h2>
        
        {/* Filter */}
        <div className="flex space-x-2">
          {['all', 'empathy', 'physics', 'consciousness'].map((f) => (
            <button
              key={f}
              onClick={() => setFilter(f)}
              className={`px-4 py-2 rounded capitalize ${
                filter === f
                  ? 'bg-blue-600'
                  : 'bg-gray-700 hover:bg-gray-600'
              }`}
            >
              {f}
            </button>
          ))}
        </div>
      </div>

      {/* NFT Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredNfts.map((nft) => (
          <NFTCard key={nft.id} nft={nft} />
        ))}
      </div>

      {filteredNfts.length === 0 && (
        <div className="text-center py-12 text-gray-400">
          No NFTs found in this category
        </div>
      )}
    </div>
  );
}

function NFTCard({ nft }) {
  const typeColors = {
    empathy: 'from-purple-600 to-pink-600',
    physics: 'from-blue-600 to-cyan-600',
    consciousness: 'from-green-600 to-teal-600',
  };

  const gradientClass = typeColors[nft.type] || 'from-gray-600 to-gray-800';

  return (
    <div className="bg-gray-800 rounded-lg overflow-hidden hover:ring-2 hover:ring-blue-500 transition">
      {/* NFT Image/Visual */}
      <div className={`h-48 bg-gradient-to-br ${gradientClass} flex items-center justify-center`}>
        <div className="text-center">
          <div className="text-6xl mb-2">🧠</div>
          <div className="text-sm uppercase tracking-wide">{nft.type}</div>
        </div>
      </div>

      {/* NFT Details */}
      <div className="p-4 space-y-2">
        <div className="flex justify-between items-start">
          <h3 className="font-semibold text-lg">
            {nft.type.charAt(0).toUpperCase() + nft.type.slice(1)} #{nft.id}
          </h3>
          <span className="bg-green-600 px-2 py-1 rounded text-xs">
            {(nft.score * 100).toFixed(0)}%
          </span>
        </div>

        <div className="text-sm text-gray-400">
          <p>IPFS: {nft.ipfsHash.slice(0, 10)}...</p>
          <p>Minted: {nft.timestamp}</p>
        </div>

        <div className="pt-2 flex space-x-2">
          <button className="flex-1 bg-blue-600 hover:bg-blue-700 py-2 rounded text-sm">
            View Details
          </button>
          <button className="flex-1 bg-gray-700 hover:bg-gray-600 py-2 rounded text-sm">
            Trade
          </button>
        </div>
      </div>
    </div>
  );
}
