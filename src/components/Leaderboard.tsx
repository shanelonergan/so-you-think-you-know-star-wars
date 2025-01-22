import React from 'react';
import { Trophy } from 'lucide-react';

export interface HighScore {
  name: string;
  score: number;
  mode: 'padawan' | 'master';
  date: string;
}

interface LeaderboardProps {
  scores: HighScore[];
}

export function Leaderboard({ scores }: LeaderboardProps) {
  const sortedScores = [...scores].sort((a, b) => b.score - a.score).slice(0, 10);

  return (
    <div className="bg-gray-900/50 rounded-lg p-6 backdrop-blur-sm">
      <div className="flex items-center justify-center gap-2 mb-6">
        <Trophy className="w-6 h-6 text-yellow-400" />
        <h2 className="text-2xl font-bold text-yellow-400">Top 10 Jedi Masters</h2>
      </div>
      
      {sortedScores.length > 0 ? (
        <div className="space-y-2">
          {sortedScores.map((score, index) => (
            <div
              key={`${score.name}-${score.date}`}
              className="flex items-center justify-between p-3 rounded-lg bg-gray-800/50 hover:bg-gray-800/70 transition-colors"
            >
              <div className="flex items-center gap-4">
                <span className="text-xl font-bold text-gray-400 w-8">
                  #{index + 1}
                </span>
                <div>
                  <p className="font-semibold">{score.name}</p>
                  <p className="text-sm text-gray-400">
                    {score.mode === 'master' ? '👑 Master' : '🎓 Padawan'}
                  </p>
                </div>
              </div>
              <div className="text-right">
                <p className="font-bold text-yellow-400">{score.score}</p>
                <p className="text-xs text-gray-400">
                  {new Date(score.date).toLocaleDateString()}
                </p>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <p className="text-center text-gray-400">No high scores yet. Be the first!</p>
      )}
    </div>
  );
}