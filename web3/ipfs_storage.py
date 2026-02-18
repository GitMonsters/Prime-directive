"""
IPFS Storage Module for Prime-directive Web3 Integration

Provides decentralized storage for:
- Model weights (physics_world_model.py output)
- Training datasets
- Consciousness states
- Benchmark results
- Research documentation
"""

try:
    import ipfshttpclient
    HAS_IPFS = True
except ImportError:
    HAS_IPFS = False
    print("Warning: ipfshttpclient not installed. IPFS features will be limited.")

import json
import os
from typing import Dict, Any, Optional
from pathlib import Path
import hashlib


class IPFSStorage:
    """Interface for IPFS storage operations"""
    
    def __init__(self, host: str = '127.0.0.1', port: int = 5001, protocol: str = 'http'):
        """
        Initialize IPFS client
        
        Args:
            host: IPFS daemon host
            port: IPFS daemon port
            protocol: Connection protocol (http/https)
        """
        if not HAS_IPFS:
            raise ImportError("ipfshttpclient not installed. Install with: pip install ipfshttpclient")
        self.client = ipfshttpclient.connect(f'/dns/{host}/tcp/{port}/{protocol}')
        
    def upload_model(self, model_path: str) -> str:
        """
        Upload a model file to IPFS
        
        Args:
            model_path: Path to the model file
            
        Returns:
            IPFS hash (CID) of the uploaded model
        """
        if not os.path.exists(model_path):
            raise FileNotFoundError(f"Model file not found: {model_path}")
        
        result = self.client.add(model_path)
        ipfs_hash = result['Hash']
        
        print(f"Model uploaded to IPFS: {ipfs_hash}")
        return ipfs_hash
    
    def download_model(self, ipfs_hash: str, output_path: str) -> str:
        """
        Download a model from IPFS
        
        Args:
            ipfs_hash: IPFS hash (CID) of the model
            output_path: Local path to save the model
            
        Returns:
            Path to the downloaded model
        """
        self.client.get(ipfs_hash, target=output_path)
        print(f"Model downloaded from IPFS to: {output_path}")
        return output_path
    
    def upload_json(self, data: Dict[Any, Any]) -> str:
        """
        Upload JSON data to IPFS
        
        Args:
            data: Dictionary to upload
            
        Returns:
            IPFS hash of the uploaded data
        """
        json_str = json.dumps(data, indent=2)
        result = self.client.add_json(data)
        return result
    
    def download_json(self, ipfs_hash: str) -> Dict[Any, Any]:
        """
        Download JSON data from IPFS
        
        Args:
            ipfs_hash: IPFS hash of the JSON data
            
        Returns:
            Parsed JSON data as dictionary
        """
        return self.client.get_json(ipfs_hash)
    
    def pin_important_data(self, ipfs_hash: str) -> bool:
        """
        Pin data to ensure it persists on the network
        
        Args:
            ipfs_hash: IPFS hash to pin
            
        Returns:
            True if successful
        """
        self.client.pin.add(ipfs_hash)
        print(f"Pinned IPFS hash: {ipfs_hash}")
        return True
    
    def unpin_data(self, ipfs_hash: str) -> bool:
        """
        Unpin data from the network
        
        Args:
            ipfs_hash: IPFS hash to unpin
            
        Returns:
            True if successful
        """
        self.client.pin.rm(ipfs_hash)
        print(f"Unpinned IPFS hash: {ipfs_hash}")
        return True
    
    def get_model_metadata(self, ipfs_hash: str) -> Dict[str, Any]:
        """
        Get metadata about a stored model
        
        Args:
            ipfs_hash: IPFS hash of the model
            
        Returns:
            Dictionary with metadata (size, links, etc.)
        """
        stat = self.client.object.stat(ipfs_hash)
        return {
            'hash': ipfs_hash,
            'size': stat['DataSize'],
            'num_links': stat['NumLinks'],
            'cumulative_size': stat['CumulativeSize']
        }
    
    def upload_directory(self, dir_path: str) -> str:
        """
        Upload an entire directory to IPFS
        
        Args:
            dir_path: Path to directory
            
        Returns:
            IPFS hash of the directory
        """
        if not os.path.isdir(dir_path):
            raise NotADirectoryError(f"Not a directory: {dir_path}")
        
        result = self.client.add(dir_path, recursive=True)
        
        # Get the root hash (last item in the result)
        root_hash = result[-1]['Hash'] if isinstance(result, list) else result['Hash']
        
        print(f"Directory uploaded to IPFS: {root_hash}")
        return root_hash
    
    def compute_file_hash(self, file_path: str) -> str:
        """
        Compute SHA-256 hash of a file for verification
        
        Args:
            file_path: Path to file
            
        Returns:
            Hex-encoded SHA-256 hash
        """
        sha256_hash = hashlib.sha256()
        with open(file_path, "rb") as f:
            for byte_block in iter(lambda: f.read(4096), b""):
                sha256_hash.update(byte_block)
        return sha256_hash.hexdigest()
    
    def close(self):
        """Close the IPFS client connection"""
        self.client.close()


# Utility functions for easy access
def upload_consciousness_state(state_data: Dict[str, Any], ipfs_client: Optional[IPFSStorage] = None) -> str:
    """
    Upload a consciousness state to IPFS
    
    Args:
        state_data: Consciousness state data
        ipfs_client: Optional IPFSStorage instance
        
    Returns:
        IPFS hash of the uploaded state
    """
    if ipfs_client is None:
        ipfs_client = IPFSStorage()
    
    ipfs_hash = ipfs_client.upload_json(state_data)
    ipfs_client.pin_important_data(ipfs_hash)
    
    return ipfs_hash


def upload_benchmark_results(results: Dict[str, Any], ipfs_client: Optional[IPFSStorage] = None) -> str:
    """
    Upload benchmark results to IPFS
    
    Args:
        results: Benchmark results data
        ipfs_client: Optional IPFSStorage instance
        
    Returns:
        IPFS hash of the uploaded results
    """
    if ipfs_client is None:
        ipfs_client = IPFSStorage()
    
    ipfs_hash = ipfs_client.upload_json(results)
    ipfs_client.pin_important_data(ipfs_hash)
    
    return ipfs_hash


if __name__ == "__main__":
    # Example usage
    try:
        storage = IPFSStorage()
        
        # Example: Upload a test file
        test_data = {
            "model": "physics_world_model",
            "version": "1.0.0",
            "accuracy": 0.95,
            "timestamp": "2026-02-18"
        }
        
        ipfs_hash = storage.upload_json(test_data)
        print(f"Test data uploaded: {ipfs_hash}")
        
        # Retrieve it back
        retrieved_data = storage.download_json(ipfs_hash)
        print(f"Retrieved data: {retrieved_data}")
        
        storage.close()
        
    except Exception as e:
        print(f"Error: {e}")
        print("Make sure IPFS daemon is running: ipfs daemon")
