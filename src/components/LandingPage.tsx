import React, { useState, useEffect } from 'react';
import { Shield, Swords } from 'lucide-react';
import { Leaderboard, type HighScore } from './Leaderboard';

interface LandingPageProps {
  onStartGame: (mode: 'padawan' | 'master') => void;
  highScores: HighScore[];
}

export function LandingPage({ onStartGame, highScores }: LandingPageProps) {
  const [showContent, setShowContent] = useState(false);

  useEffect(() => {
    const timer = setTimeout(() => {
      setShowContent(true);
    }, 3500);

    return () => clearTimeout(timer);
  }, []);

  const content = (
    <>
      <h1 className="text-yellow-400 text-5xl font-bold mb-8">
        You think you know Star Wars?
      </h1>
      <h2 className="text-yellow-400 text-3xl mb-12">
        Prove it!
      </h2>
      
      <div className="flex justify-center gap-6 mb-12">
        {showContent ? (
          <>
            <button
              onClick={() => onStartGame('padawan')}
              className="bg-green-600 hover:bg-green-500 text-white px-8 py-4 rounded-lg text-xl font-semibold transition-colors flex items-center justify-center gap-3"
            >
              <Shield className="w-6 h-6" />
              Padawan Mode
            </button>
            
            <button
              onClick={() => onStartGame('master')}
              className="bg-red-600 hover:bg-red-500 text-white px-8 py-4 rounded-lg text-xl font-semibold transition-colors flex items-center justify-center gap-3"
            >
              <Swords className="w-6 h-6" />
              Master Mode
            </button>
          </>
        ) : (
          <>
            <div className="bg-green-600 text-white px-8 py-4 rounded-lg text-xl font-semibold flex items-center justify-center gap-3">
              <Shield className="w-6 h-6" />
              Padawan Mode
            </div>
            
            <div className="bg-red-600 text-white px-8 py-4 rounded-lg text-xl font-semibold flex items-center justify-center gap-3">
              <Swords className="w-6 h-6" />
              Master Mode
            </div>
          </>
        )}
      </div>

      <div className="text-gray-400 max-w-md mx-auto mb-12">
        <div className="flex justify-center gap-8">
          <p>
            <strong className="text-green-400">Padawan Mode:</strong> Multiple choice
          </p>
          <p>
            <strong className="text-red-400">Master Mode:</strong> Type the name
          </p>
        </div>
      </div>

      <Leaderboard scores={highScores} />
    </>
  );

  return (
    <div className="min-h-screen bg-black flex items-center justify-center px-4">
      <div className="starfield"></div>
      <div className="max-w-4xl w-full">
        <div className={`absolute inset-0 ${!showContent ? 'pointer-events-auto opacity-100' : 'pointer-events-none opacity-0'} transition-opacity duration-500`}>
          <div className="star-wars-container">
            <div className="star-wars-crawl">
              <div className="text-center">
                {content}
              </div>
            </div>
          </div>
        </div>

        <div className={`text-center mb-12 ${showContent ? 'opacity-100' : 'opacity-0'} transition-opacity duration-500`}>
          {content}
        </div>
      </div>
    </div>
  );
}