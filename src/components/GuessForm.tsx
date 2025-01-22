import React from 'react';
import { Swords } from 'lucide-react';

interface GuessFormProps {
  guess: string;
  onGuessChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onGuessSubmit: (e: React.FormEvent) => void;
  onChoiceSelect: (choice: string) => void;
  message: string;
  disabled?: boolean;
  mode: 'padawan' | 'master';
  choices: string[];
}

export function GuessForm({
  guess,
  onGuessChange,
  onGuessSubmit,
  onChoiceSelect,
  message,
  disabled,
  mode,
  choices
}: GuessFormProps) {
  if (mode === 'padawan') {
    return (
      <>
        <div className="grid grid-cols-2 gap-4">
          {choices.map((choice, index) => (
            <button
              key={index}
              onClick={() => onChoiceSelect(choice)}
              disabled={disabled}
              className="bg-gray-700 px-4 py-3 rounded-lg hover:bg-gray-600 transition-colors text-left disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {choice}
            </button>
          ))}
        </div>
        {message && (
          <div
            className={`mt-4 text-center ${
              message.includes('Correct')
                ? 'text-green-400'
                : message.includes('was')
                ? 'text-yellow-400'
                : 'text-red-400'
            }`}
          >
            {message}
          </div>
        )}
      </>
    );
  }

  return (
    <>
      <form onSubmit={onGuessSubmit} className="flex gap-2">
        <input
          type="text"
          value={guess}
          onChange={onGuessChange}
          placeholder="Enter character name..."
          className="flex-1 px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 focus:outline-none focus:border-yellow-400 disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={disabled}
        />
        <button
          type="submit"
          className="bg-yellow-400 text-black px-6 py-2 rounded-lg hover:bg-yellow-300 transition-colors flex items-center disabled:opacity-50 disabled:cursor-not-allowed"
          disabled={disabled}
        >
          <Swords className="mr-2" />
          Guess
        </button>
      </form>

      {message && (
        <div
          className={`mt-4 text-center ${
            message.includes('Correct')
              ? 'text-green-400'
              : message.includes('was')
              ? 'text-yellow-400'
              : 'text-red-400'
          }`}
        >
          {message}
        </div>
      )}
    </>
  );
}