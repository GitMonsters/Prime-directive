import React, { useState, useEffect } from 'react';

export default function ComputeMarket() {
  const [activeTasks, setActiveTasks] = useState([]);
  const [availableNodes, setAvailableNodes] = useState([]);
  const [showSubmitForm, setShowSubmitForm] = useState(false);

  useEffect(() => {
    // Mock data - in production, fetch from blockchain
    setActiveTasks([
      { id: 'task_1', type: 'Training', reward: 1000, status: 'In Progress' },
      { id: 'task_2', type: 'Inference', reward: 500, status: 'Pending' },
      { id: 'task_3', type: 'Benchmark', reward: 800, status: 'Completed' },
    ]);

    setAvailableNodes([
      { id: 'node_1', gpus: 4, reputation: 4.8, tasksCompleted: 145 },
      { id: 'node_2', gpus: 8, reputation: 4.9, tasksCompleted: 203 },
      { id: 'node_3', gpus: 2, reputation: 4.5, tasksCompleted: 87 },
    ]);
  }, []);

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-3xl font-bold">Compute Marketplace</h2>
        <button
          onClick={() => setShowSubmitForm(!showSubmitForm)}
          className="bg-blue-600 hover:bg-blue-700 px-6 py-2 rounded"
        >
          Submit New Task
        </button>
      </div>

      {showSubmitForm && <SubmitTaskForm onClose={() => setShowSubmitForm(false)} />}

      {/* Active Tasks */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-xl font-semibold mb-4">Active Tasks</h3>
        <div className="space-y-3">
          {activeTasks.map((task) => (
            <TaskCard key={task.id} task={task} />
          ))}
        </div>
      </div>

      {/* Available Nodes */}
      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-xl font-semibold mb-4">Available Compute Nodes</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {availableNodes.map((node) => (
            <NodeCard key={node.id} node={node} />
          ))}
        </div>
      </div>
    </div>
  );
}

function TaskCard({ task }) {
  const statusColor = {
    'Pending': 'text-yellow-400',
    'In Progress': 'text-blue-400',
    'Completed': 'text-green-400',
  }[task.status];

  return (
    <div className="flex justify-between items-center p-4 bg-gray-700 rounded">
      <div className="space-y-1">
        <p className="font-semibold">{task.id}</p>
        <p className="text-sm text-gray-400">{task.type}</p>
      </div>
      <div className="text-right space-y-1">
        <p className="font-bold text-green-400">{task.reward} PRIME</p>
        <p className={`text-sm ${statusColor}`}>{task.status}</p>
      </div>
    </div>
  );
}

function NodeCard({ node }) {
  return (
    <div className="bg-gray-700 rounded-lg p-4 space-y-2">
      <div className="flex justify-between items-center">
        <span className="font-semibold">{node.id}</span>
        <span className="text-yellow-400">★ {node.reputation}</span>
      </div>
      <div className="text-sm text-gray-400">
        <p>GPUs: {node.gpus}</p>
        <p>Tasks: {node.tasksCompleted}</p>
      </div>
      <button className="w-full bg-blue-600 hover:bg-blue-700 py-2 rounded text-sm">
        Assign Task
      </button>
    </div>
  );
}

function SubmitTaskForm({ onClose }) {
  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <h3 className="text-xl font-semibold mb-4">Submit New Task</h3>
      <div className="space-y-4">
        <div>
          <label className="block text-sm mb-2">Task Type</label>
          <select className="w-full bg-gray-700 px-4 py-2 rounded">
            <option>Training</option>
            <option>Inference</option>
            <option>Benchmark</option>
          </select>
        </div>
        <div>
          <label className="block text-sm mb-2">IPFS Hash</label>
          <input
            type="text"
            placeholder="QmXXXXX..."
            className="w-full bg-gray-700 px-4 py-2 rounded"
          />
        </div>
        <div>
          <label className="block text-sm mb-2">Reward (PRIME)</label>
          <input
            type="number"
            placeholder="1000"
            className="w-full bg-gray-700 px-4 py-2 rounded"
          />
        </div>
        <div className="flex space-x-3">
          <button className="flex-1 bg-blue-600 hover:bg-blue-700 py-2 rounded">
            Submit
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
