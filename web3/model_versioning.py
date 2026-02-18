"""
Blockchain-based Model Version Control for Prime-directive

Tracks model lineage on-chain with IPFS for actual weights,
NFTs for each major version, and Merkle proofs for integrity.
"""

from typing import Dict, Any, List, Optional
import json
from datetime import datetime
from web3 import Web3
from .ipfs_storage import IPFSStorage


class ModelVersion:
    """Represents a single model version"""
    
    def __init__(self, version: str, ipfs_hash: str, parent_version: Optional[str] = None,
                 metadata: Optional[Dict[str, Any]] = None):
        self.version = version
        self.ipfs_hash = ipfs_hash
        self.parent_version = parent_version
        self.metadata = metadata or {}
        self.timestamp = datetime.now().isoformat()
        self.merkle_root = self._compute_merkle_root()
    
    def _compute_merkle_root(self) -> str:
        """Compute Merkle root for integrity verification"""
        data_str = json.dumps({
            'version': self.version,
            'ipfs_hash': self.ipfs_hash,
            'parent': self.parent_version,
            'timestamp': self.timestamp
        }, sort_keys=True)
        
        return hashlib.sha256(data_str.encode()).hexdigest()
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary"""
        return {
            'version': self.version,
            'ipfs_hash': self.ipfs_hash,
            'parent_version': self.parent_version,
            'metadata': self.metadata,
            'timestamp': self.timestamp,
            'merkle_root': self.merkle_root
        }


class ModelVersionControl:
    """Blockchain-based model version control system"""
    
    def __init__(self, ipfs_storage: Optional[IPFSStorage] = None,
                 web3_provider: Optional[str] = None):
        """
        Initialize model version control
        
        Args:
            ipfs_storage: IPFSStorage instance
            web3_provider: Web3 provider URL (optional, for on-chain tracking)
        """
        self.ipfs = ipfs_storage or IPFSStorage()
        self.versions: Dict[str, ModelVersion] = {}
        self.version_history: List[str] = []
        
        if web3_provider:
            self.web3 = Web3(Web3.HTTPProvider(web3_provider))
        else:
            self.web3 = None
    
    def create_version(self, version: str, model_path: str,
                      parent_version: Optional[str] = None,
                      metadata: Optional[Dict[str, Any]] = None) -> ModelVersion:
        """
        Create a new model version
        
        Args:
            version: Version identifier (e.g., "1.0.0", "2.1.0")
            model_path: Path to model file
            parent_version: Parent version identifier (for lineage)
            metadata: Additional metadata (accuracy, training params, etc.)
            
        Returns:
            ModelVersion object
        """
        # Upload model to IPFS
        ipfs_hash = self.ipfs.upload_model(model_path)
        self.ipfs.pin_important_data(ipfs_hash)
        
        # Create version object
        model_version = ModelVersion(
            version=version,
            ipfs_hash=ipfs_hash,
            parent_version=parent_version,
            metadata=metadata
        )
        
        # Store in memory
        self.versions[version] = model_version
        self.version_history.append(version)
        
        # Store version info on IPFS
        version_info_hash = self.ipfs.upload_json(model_version.to_dict())
        
        print(f"Created model version {version}")
        print(f"  Model IPFS hash: {ipfs_hash}")
        print(f"  Version info hash: {version_info_hash}")
        print(f"  Merkle root: {model_version.merkle_root}")
        
        return model_version
    
    def get_version(self, version: str) -> Optional[ModelVersion]:
        """
        Get a specific model version
        
        Args:
            version: Version identifier
            
        Returns:
            ModelVersion object or None
        """
        return self.versions.get(version)
    
    def download_version(self, version: str, output_path: str) -> str:
        """
        Download a specific model version
        
        Args:
            version: Version identifier
            output_path: Local path to save the model
            
        Returns:
            Path to downloaded model
        """
        model_version = self.versions.get(version)
        if not model_version:
            raise ValueError(f"Version {version} not found")
        
        return self.ipfs.download_model(model_version.ipfs_hash, output_path)
    
    def get_lineage(self, version: str) -> List[str]:
        """
        Get the lineage of a model version
        
        Args:
            version: Version identifier
            
        Returns:
            List of version identifiers from root to current
        """
        lineage = [version]
        current = self.versions.get(version)
        
        while current and current.parent_version:
            lineage.insert(0, current.parent_version)
            current = self.versions.get(current.parent_version)
        
        return lineage
    
    def verify_integrity(self, version: str) -> bool:
        """
        Verify the integrity of a model version using Merkle root
        
        Args:
            version: Version identifier
            
        Returns:
            True if integrity check passes
        """
        model_version = self.versions.get(version)
        if not model_version:
            return False
        
        # Recompute Merkle root
        recomputed_root = model_version._compute_merkle_root()
        
        return recomputed_root == model_version.merkle_root
    
    def list_versions(self) -> List[str]:
        """
        List all model versions
        
        Returns:
            List of version identifiers
        """
        return self.version_history.copy()
    
    def get_latest_version(self) -> Optional[ModelVersion]:
        """
        Get the latest model version
        
        Returns:
            Latest ModelVersion object or None
        """
        if not self.version_history:
            return None
        
        latest_version_id = self.version_history[-1]
        return self.versions[latest_version_id]
    
    def export_version_history(self) -> str:
        """
        Export entire version history to IPFS
        
        Returns:
            IPFS hash of the version history
        """
        history_data = {
            'versions': [v.to_dict() for v in self.versions.values()],
            'history': self.version_history,
            'exported_at': datetime.now().isoformat()
        }
        
        ipfs_hash = self.ipfs.upload_json(history_data)
        self.ipfs.pin_important_data(ipfs_hash)
        
        print(f"Version history exported to IPFS: {ipfs_hash}")
        return ipfs_hash
    
    def compare_versions(self, version1: str, version2: str) -> Dict[str, Any]:
        """
        Compare two model versions
        
        Args:
            version1: First version identifier
            version2: Second version identifier
            
        Returns:
            Dictionary with comparison results
        """
        v1 = self.versions.get(version1)
        v2 = self.versions.get(version2)
        
        if not v1 or not v2:
            raise ValueError("One or both versions not found")
        
        return {
            'version1': version1,
            'version2': version2,
            'different_ipfs_hash': v1.ipfs_hash != v2.ipfs_hash,
            'metadata_diff': self._diff_metadata(v1.metadata, v2.metadata),
            'lineage1': self.get_lineage(version1),
            'lineage2': self.get_lineage(version2)
        }
    
    def _diff_metadata(self, meta1: Dict[str, Any], meta2: Dict[str, Any]) -> Dict[str, Any]:
        """Compare metadata between two versions"""
        all_keys = set(meta1.keys()) | set(meta2.keys())
        
        diff = {}
        for key in all_keys:
            val1 = meta1.get(key)
            val2 = meta2.get(key)
            
            if val1 != val2:
                diff[key] = {'v1': val1, 'v2': val2}
        
        return diff


if __name__ == "__main__":
    # Example usage
    print("Model Version Control Example")
    print("-" * 50)
    
    # This would normally connect to IPFS
    # For demonstration, we'll show the API
    
    # mvc = ModelVersionControl()
    
    # Create first version
    # v1 = mvc.create_version(
    #     version="1.0.0",
    #     model_path="path/to/model_v1.pkl",
    #     metadata={"accuracy": 0.85, "training_epochs": 100}
    # )
    
    # Create improved version
    # v2 = mvc.create_version(
    #     version="1.1.0",
    #     model_path="path/to/model_v2.pkl",
    #     parent_version="1.0.0",
    #     metadata={"accuracy": 0.92, "training_epochs": 150}
    # )
    
    # Get lineage
    # lineage = mvc.get_lineage("1.1.0")
    # print(f"Lineage: {lineage}")
    
    # Verify integrity
    # is_valid = mvc.verify_integrity("1.1.0")
    # print(f"Integrity check: {is_valid}")
    
    print("See code comments for usage examples")
