import React from 'react';
import { Shield, Swords } from 'lucide-react';

interface GameModeProps {
  mode: 'padawan' | 'master';
  onModeChange: (mode: 'padawan' | 'master') => void;
}

export function GameMode({ mode, onModeChange }: GameModeProps) {
  return (
    <div className="flex justify-center gap-4 mb-8">
      <button
        onClick={() => onModeChange('padawan')}
        className={`px-6 py-2 rounded-lg flex items-center gap-2 transition-colors ${
          mode === 'padawan'
            ? 'bg-green-600 text-white'
            : 'bg-gray-700 hover:bg-gray-600'
        }`}
      >
        <Shield className="w-5 h-5" />
        Padawan Mode
      </button>
      <button
        onClick={() => onModeChange('master')}
        className={`px-6 py-2 rounded-lg flex items-center gap-2 transition-colors ${
          mode === 'master'
            ? 'bg-red-600 text-white'
            : 'bg-gray-700 hover:bg-gray-600'
        }`}
      >
        <Swords className="w-5 h-5" />
        Master Mode
      </button>
    </div>
  );
}