import React, { useState, useEffect } from 'react';

export default function GovernancePanel() {
  const [proposals, setProposals] = useState([]);
  const [showCreateForm, setShowCreateForm] = useState(false);

  useEffect(() => {
    // Mock proposals - in production, fetch from blockchain
    setProposals([
      {
        id: 1,
        title: 'Increase Compute Rewards by 20%',
        description: 'Proposal to increase GPU hour rewards from 1000 to 1200 PRIME',
        type: 'TreasuryAllocation',
        votesFor: 150000,
        votesAgainst: 45000,
        endTime: '2026-02-25',
        status: 'Active',
      },
      {
        id: 2,
        title: 'Fund GAIA Benchmark Improvements',
        description: 'Allocate 100k PRIME for improving GAIA benchmark accuracy',
        type: 'ExperimentDirection',
        votesFor: 120000,
        votesAgainst: 30000,
        endTime: '2026-02-24',
        status: 'Active',
      },
      {
        id: 3,
        title: 'Enable Cross-Chain Bridge to Solana',
        description: 'Deploy bridge contract for PRIME token on Solana',
        type: 'TreasuryAllocation',
        votesFor: 200000,
        votesAgainst: 20000,
        endTime: '2026-02-18',
        status: 'Passed',
      },
    ]);
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-bold">DAO Governance</h2>
        <button
          onClick={() => setShowCreateForm(!showCreateForm)}
          className="bg-blue-600 hover:bg-blue-700 px-6 py-2 rounded"
        >
          Create Proposal
        </button>
      </div>

      {showCreateForm && <CreateProposalForm onClose={() => setShowCreateForm(false)} />}

      {/* Treasury Stats */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <StatCard title="Treasury Balance" value="250M PRIME" />
        <StatCard title="Active Proposals" value="2" />
        <StatCard title="Participation Rate" value="67%" />
      </div>

      {/* Proposals List */}
      <div className="space-y-4">
        {proposals.map((proposal) => (
          <ProposalCard key={proposal.id} proposal={proposal} />
        ))}
      </div>
    </div>
  );
}

function ProposalCard({ proposal }) {
  const totalVotes = proposal.votesFor + proposal.votesAgainst;
  const forPercentage = totalVotes > 0 
    ? (proposal.votesFor / totalVotes) * 100 
    : 0;

  const statusColors = {
    'Active': 'bg-blue-600',
    'Passed': 'bg-green-600',
    'Rejected': 'bg-red-600',
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <div className="flex justify-between items-start mb-4">
        <div className="flex-1">
          <div className="flex items-center space-x-3 mb-2">
            <h3 className="text-xl font-semibold">{proposal.title}</h3>
            <span className={`${statusColors[proposal.status]} px-3 py-1 rounded text-sm`}>
              {proposal.status}
            </span>
          </div>
          <p className="text-gray-400 text-sm mb-2">{proposal.description}</p>
          <p className="text-gray-500 text-xs">
            Type: {proposal.type} • Ends: {proposal.endTime}
          </p>
        </div>
      </div>

      {/* Voting Progress */}
      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2">
          <span>For: {proposal.votesFor.toLocaleString()}</span>
          <span>Against: {proposal.votesAgainst.toLocaleString()}</span>
        </div>
        <div className="w-full bg-gray-700 rounded-full h-3">
          <div
            className="bg-green-500 h-3 rounded-full transition-all"
            style={{ width: `${forPercentage}%` }}
          />
        </div>
        <div className="text-center text-sm mt-1 text-gray-400">
          {forPercentage.toFixed(1)}% in favor
        </div>
      </div>

      {/* Voting Buttons */}
      {proposal.status === 'Active' && (
        <div className="flex space-x-3">
          <button className="flex-1 bg-green-600 hover:bg-green-700 py-2 rounded">
            Vote For
          </button>
          <button className="flex-1 bg-red-600 hover:bg-red-700 py-2 rounded">
            Vote Against
          </button>
        </div>
      )}
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

function CreateProposalForm({ onClose }) {
  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <h3 className="text-xl font-semibold mb-4">Create New Proposal</h3>
      <div className="space-y-4">
        <div>
          <label className="block text-sm mb-2">Title</label>
          <input
            type="text"
            placeholder="Proposal title..."
            className="w-full bg-gray-700 px-4 py-2 rounded"
          />
        </div>
        <div>
          <label className="block text-sm mb-2">Description</label>
          <textarea
            placeholder="Detailed description..."
            rows={4}
            className="w-full bg-gray-700 px-4 py-2 rounded"
          />
        </div>
        <div>
          <label className="block text-sm mb-2">Proposal Type</label>
          <select className="w-full bg-gray-700 px-4 py-2 rounded">
            <option>ExperimentDirection</option>
            <option>TreasuryAllocation</option>
            <option>MilestoneTracking</option>
          </select>
        </div>
        <div className="flex space-x-3">
          <button className="flex-1 bg-blue-600 hover:bg-blue-700 py-2 rounded">
            Submit (10k PRIME required)
          </button>
          <button
            onClick={onClose}
            className="flex-1 bg-gray-700 hover:bg-gray-600 py-2 rounded"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
