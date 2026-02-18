import React, { useState, useEffect } from 'react';
import { useAccount, useConnect, useDisconnect } from 'wagmi';
import ComputeMarket from '../components/ComputeMarket';
import ConsciousnessGallery from '../components/ConsciousnessGallery';
import GovernancePanel from '../components/GovernancePanel';

export default function Home() {
  const { address, isConnected } = useAccount();
  const { connect } = useConnect();
  const { disconnect } = useDisconnect();
  const [activeTab, setActiveTab] = useState('dashboard');
  const [primeBalance, setPrimeBalance] = useState(0);

  useEffect(() => {
    if (isConnected && address) {
      // Fetch PRIME balance
      // In production: call contract to get balance
      setPrimeBalance(10000); // Mock balance
    }
  }, [isConnected, address]);

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center space-x-4">
              <h1 className="text-2xl font-bold">Prime-directive AGI</h1>
              <span className="text-sm text-gray-400">Web3 Consciousness Platform</span>
            </div>
            
            <div className="flex items-center space-x-4">
              {isConnected ? (
                <>
                  <div className="bg-gray-700 px-4 py-2 rounded">
                    <span className="text-sm text-gray-300">PRIME Balance: </span>
                    <span className="font-bold">{primeBalance.toLocaleString()}</span>
                  </div>
                  <div className="bg-gray-700 px-4 py-2 rounded">
                    <span className="text-xs">{address?.slice(0, 6)}...{address?.slice(-4)}</span>
                  </div>
                  <button
                    onClick={() => disconnect()}
                    className="bg-red-600 hover:bg-red-700 px-4 py-2 rounded"
                  >
                    Disconnect
                  </button>
                </>
              ) : (
                <button
                  onClick={() => connect()}
                  className="bg-blue-600 hover:bg-blue-700 px-6 py-2 rounded"
                >
                  Connect Wallet
                </button>
              )}
            </div>
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className="bg-gray-800 border-b border-gray-700">
        <div className="container mx-auto px-4">
          <div className="flex space-x-1">
            {['dashboard', 'compute', 'gallery', 'governance'].map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`px-6 py-3 capitalize ${
                  activeTab === tab
                    ? 'bg-gray-900 border-b-2 border-blue-500'
                    : 'hover:bg-gray-700'
                }`}
              >
                {tab}
              </button>
            ))}
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        {activeTab === 'dashboard' && <Dashboard primeBalance={primeBalance} />}
        {activeTab === 'compute' && <ComputeMarket />}
        {activeTab === 'gallery' && <ConsciousnessGallery />}
        {activeTab === 'governance' && <GovernancePanel />}
      </main>
    </div>
  );
}

function Dashboard({ primeBalance }) {
  return (
    <div className="space-y-6">
      <h2 className="text-3xl font-bold">Dashboard</h2>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Stats Cards */}
        <StatCard title="PRIME Balance" value={primeBalance.toLocaleString()} />
        <StatCard title="Active Tasks" value="12" />
        <StatCard title="NFTs Owned" value="5" />
      </div>

      {/* Recent Activity */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-xl font-semibold mb-4">Recent Activity</h3>
        <div className="space-y-3">
          <ActivityItem
            type="compute"
            description="Completed training task #1234"
            reward="+1000 PRIME"
          />
          <ActivityItem
            type="nft"
            description="Minted Consciousness NFT #42"
            reward="NFT"
          />
          <ActivityItem
            type="governance"
            description="Voted on Proposal #7"
            reward="+100 PRIME"
          />
        </div>
      </div>
    </div>
  );
}

function StatCard({ title, value }) {
  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <h3 className="text-gray-400 text-sm mb-2">{title}</h3>
      <p className="text-3xl font-bold">{value}</p>
    </div>
  );
}

function ActivityItem({ type, description, reward }) {
  const iconColor = {
    compute: 'text-blue-400',
    nft: 'text-purple-400',
    governance: 'text-green-400',
  }[type];

  return (
    <div className="flex justify-between items-center p-3 bg-gray-700 rounded">
      <div className="flex items-center space-x-3">
        <span className={`text-2xl ${iconColor}`}>●</span>
        <span>{description}</span>
      </div>
      <span className="text-green-400 font-semibold">{reward}</span>
    </div>
  );
}
