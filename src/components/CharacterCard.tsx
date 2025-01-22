import React, { useState } from 'react';
import { HelpCircle, RefreshCcw, Flag } from 'lucide-react';

interface CharacterCardProps {
  character: any;
  characterInfo: any;
  showHint: boolean;
  onShowHint: () => void;
  onSkip: () => void;
  onGiveUp: () => void;
  revealAnswer: boolean;
  children: React.ReactNode;
}

export function CharacterCard({ 
  character, 
  characterInfo, 
  showHint, 
  onShowHint, 
  onSkip, 
  onGiveUp,
  revealAnswer,
  children 
}: CharacterCardProps) {
  const [currentTrivia, setCurrentTrivia] = useState<string>("");
  const characterId = character.url.split('/').filter(Boolean).pop();
  
  const getRandomTrivia = () => {
    if (!characterInfo) return "Loading trivia...";
    
    const triviaOptions = [
      `This character comes from the planet ${characterInfo.homeworld}.`,
      `They are of ${characterInfo.species} species.`,
      `You can spot them in the film "${characterInfo.film}".`,
      characterInfo.vehicles > 0 ? `They have piloted ${characterInfo.vehicles} different vehicles.` : null,
      characterInfo.starships > 0 ? `They have flown ${characterInfo.starships} different starships.` : null,
    ].filter(Boolean);

    const randomIndex = Math.floor(Math.random() * triviaOptions.length);
    return triviaOptions[randomIndex];
  };

  const handleShowHint = () => {
    setCurrentTrivia(getRandomTrivia());
    onShowHint();
  };
  
  return (
    <div className="bg-gray-900 rounded-lg p-8 shadow-lg">
      <div className="mb-6">
        <div className="text-center mb-4 h-[600px] flex items-center justify-center bg-gray-800 rounded-lg">
          <img
            src={`https://starwars-visualguide.com/assets/img/characters/${characterId}.jpg`}
            alt="Star Wars Character"
            className="rounded-lg h-full w-auto object-contain"
            onError={(e) => {
              e.currentTarget.onerror = null;
              e.currentTarget.src = `https://source.unsplash.com/800x600/?star%20wars,alien`;
            }}
          />
        </div>
        
        {showHint && (
          <div className="bg-gray-800 p-4 rounded-lg mb-4 text-center">
            <p className="text-lg text-yellow-400">{currentTrivia}</p>
          </div>
        )}

        {children}

        <div className="flex justify-center gap-4 mt-4">
          <button
            onClick={handleShowHint}
            className="bg-gray-700 px-4 py-2 rounded-lg hover:bg-gray-600 transition-colors flex items-center disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={showHint || revealAnswer}
          >
            <HelpCircle className="mr-2" />
            Show Hint
          </button>
          <button
            onClick={onGiveUp}
            className="bg-red-700 px-4 py-2 rounded-lg hover:bg-red-600 transition-colors flex items-center disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={revealAnswer}
          >
            <Flag className="mr-2" />
            I Give Up
          </button>
          <button
            onClick={onSkip}
            className="bg-gray-700 px-4 py-2 rounded-lg hover:bg-gray-600 transition-colors flex items-center"
          >
            <RefreshCcw className="mr-2" />
            Skip
          </button>
        </div>
      </div>
    </div>
  );
}