import React from 'react';
import { Trophy, Heart } from 'lucide-react';

interface HeaderProps {
  score: number;
  lives: number;
}

export function Header({ score, lives }: HeaderProps) {
  return (
    <div className="text-center mb-8">
      <h1 className="text-4xl font-bold text-yellow-400 mb-2">Star Wars Character Quiz</h1>
      <div className="flex justify-center items-center space-x-8 text-xl">
        <div className="flex items-center">
          <Trophy className="text-yellow-400 mr-2" />
          <span>{score}</span>
        </div>
        <div className="flex items-center">
          <Heart className="text-red-500 mr-2" />
          <span>{lives}</span>
        </div>
      </div>
    </div>
  );
}