import React, { useState } from 'react';
import { Trophy, ArrowRight } from 'lucide-react';
import type { HighScore } from './Leaderboard';

interface GameOverProps {
  score: number;
  mode: 'padawan' | 'master';
  onReset: () => void;
  onSaveScore: (score: HighScore) => void;
}

export function GameOver({ score, mode, onReset, onSaveScore }: GameOverProps) {
  const [name, setName] = useState('');
  const [saved, setSaved] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim()) return;

    onSaveScore({
      name: name.trim(),
      score,
      mode,
      date: new Date().toISOString()
    });
    setSaved(true);
  };

  return (
    <div className="min-h-screen bg-black flex items-center justify-center px-4">
      <div className="max-w-md w-full text-center">
        <h2 className="text-4xl font-bold mb-6 text-red-500">Game Over!</h2>
        <div className="flex items-center justify-center mb-8">
          <Trophy className="text-yellow-400 mr-3 h-8 w-8" />
          <p className="text-2xl">Final Score: {score}</p>
        </div>

        {!saved ? (
          <form onSubmit={handleSubmit} className="mb-8">
            <div className="mb-4">
              <input
                type="text"
                value={name}
                onChange={(e) => setName(e.target.value)}
                placeholder="Enter your name"
                maxLength={20}
                className="w-full px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 focus:outline-none focus:border-yellow-400 text-center"
              />
            </div>
            <button
              type="submit"
              disabled={!name.trim()}
              className="bg-yellow-400 text-black px-6 py-3 rounded-lg hover:bg-yellow-300 transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 w-full"
            >
              Save Score
              <ArrowRight className="w-5 h-5" />
            </button>
          </form>
        ) : (
          <div className="mb-8">
            <p className="text-green-400 mb-4">Score saved successfully!</p>
          </div>
        )}

        <button
          onClick={onReset}
          className="bg-gray-700 text-white px-6 py-3 rounded-lg hover:bg-gray-600 transition-colors w-full"
        >
          Play Again
        </button>
      </div>
    </div>
  );
}