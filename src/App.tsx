import React, { useState, useEffect } from 'react';
import { Header } from './components/Header';
import { CharacterCard } from './components/CharacterCard';
import { GuessForm } from './components/GuessForm';
import { GameOver } from './components/GameOver';
import { GameMode } from './components/GameMode';
import { LandingPage } from './components/LandingPage';
import { isCloseMatch } from './utils/stringUtils';
import type { HighScore } from './components/Leaderboard';

interface Character {
  name: string;
  url: string;
}

interface CharacterInfo {
  homeworld: string;
  species: string;
  film: string;
  vehicles: number;
  starships: number;
}

const STORAGE_KEY = 'star-wars-quiz-highscores';

function App() {
  const [gameStarted, setGameStarted] = useState(false);
  const [mode, setMode] = useState<'padawan' | 'master'>('padawan');
  const [character, setCharacter] = useState<Character | null>(null);
  const [characterInfo, setCharacterInfo] = useState<CharacterInfo | null>(null);
  const [guess, setGuess] = useState('');
  const [score, setScore] = useState(0);
  const [lives, setLives] = useState(3);
  const [message, setMessage] = useState('');
  const [gameOver, setGameOver] = useState(false);
  const [showHint, setShowHint] = useState(false);
  const [revealAnswer, setRevealAnswer] = useState(false);
  const [choices, setChoices] = useState<string[]>([]);
  const [highScores, setHighScores] = useState<HighScore[]>(() => {
    const saved = localStorage.getItem(STORAGE_KEY);
    return saved ? JSON.parse(saved) : [];
  });

  useEffect(() => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(highScores));
  }, [highScores]);

  const fetchRandomCharacter = async () => {
    try {
      const page = Math.floor(Math.random() * 8) + 1;
      const response = await fetch(`https://swapi.dev/api/people/?page=${page}`);
      const data = await response.json();
      const randomCharacter = data.results[Math.floor(Math.random() * data.results.length)];
      setCharacter(randomCharacter);
      setRevealAnswer(false);
      setShowHint(false);
      setMessage('');
      
      // Fetch additional character info
      const [homeworldRes, filmsRes] = await Promise.all([
        fetch(randomCharacter.homeworld),
        fetch(randomCharacter.films[0])
      ]);
      
      const [homeworldData, filmData] = await Promise.all([
        homeworldRes.json(),
        filmsRes.json()
      ]);

      const speciesName = randomCharacter.species.length > 0 
        ? await fetch(randomCharacter.species[0]).then(res => res.json()).then(data => data.name)
        : 'Human';

      setCharacterInfo({
        homeworld: homeworldData.name,
        species: speciesName,
        film: filmData.title,
        vehicles: randomCharacter.vehicles.length,
        starships: randomCharacter.starships.length
      });

      if (mode === 'padawan') {
        const otherCharacters = data.results
          .filter((c: Character) => c.name !== randomCharacter.name)
          .sort(() => Math.random() - 0.5)
          .slice(0, 3);
        
        const allChoices = [...otherCharacters, randomCharacter]
          .map((c: Character) => c.name)
          .sort(() => Math.random() - 0.5);
        
        setChoices(allChoices);
      }
    } catch (error) {
      console.error('Error fetching character:', error);
    }
  };

  useEffect(() => {
    if (gameStarted) {
      fetchRandomCharacter();
    }
  }, [gameStarted]);

  const handleGuess = (guessValue: string) => {
    if (!character) return;

    if (guessValue.toLowerCase() === character.name.toLowerCase()) {
      setScore(score + (mode === 'master' ? 100 : 50));
      setMessage('Correct! Well done!');
      setTimeout(fetchRandomCharacter, 1500);
    } else if (mode === 'master' && isCloseMatch(guessValue, character.name)) {
      setScore(score + 75); // Slightly lower score for close matches
      setMessage(`Sci-fi spelling is hard... close enough! The correct spelling was: ${character.name}`);
      setTimeout(fetchRandomCharacter, 2000);
    } else {
      setLives(lives - 1);
      setMessage('Wrong guess! Try again!');
      if (lives <= 1) {
        setGameOver(true);
      }
    }
    setGuess('');
  };

  const handleGuessSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    handleGuess(guess);
  };

  const handleChoiceSelect = (choice: string) => {
    handleGuess(choice);
  };

  const handleGiveUp = () => {
    if (character) {
      setMessage(`The correct answer was: ${character.name}`);
      setRevealAnswer(true);
      setTimeout(fetchRandomCharacter, 2000);
    }
  };

  const handleSaveScore = (newScore: HighScore) => {
    setHighScores(prev => [...prev, newScore]);
  };

  const handleReset = () => {
    setScore(0);
    setLives(3);
    setGameOver(false);
    setGameStarted(false);
    setShowHint(false);
    setRevealAnswer(false);
    setMessage('');
  };

  const handleStartGame = (selectedMode: 'padawan' | 'master') => {
    setMode(selectedMode);
    setGameStarted(true);
  };

  if (!gameStarted) {
    return <LandingPage onStartGame={handleStartGame} highScores={highScores} />;
  }

  if (gameOver) {
    return (
      <GameOver 
        score={score} 
        mode={mode}
        onReset={handleReset} 
        onSaveScore={handleSaveScore}
      />
    );
  }

  return (
    <div className="min-h-screen bg-black text-white p-8">
      <Header score={score} lives={lives} />
      <GameMode mode={mode} onModeChange={setMode} />
      
      {character && (
        <div className="max-w-2xl mx-auto">
          <CharacterCard
            character={character}
            characterInfo={characterInfo}
            showHint={showHint}
            onShowHint={() => setShowHint(true)}
            onSkip={fetchRandomCharacter}
            onGiveUp={handleGiveUp}
            revealAnswer={revealAnswer}
          >
            <GuessForm
              guess={guess}
              onGuessChange={(e) => setGuess(e.target.value)}
              onGuessSubmit={handleGuessSubmit}
              onChoiceSelect={handleChoiceSelect}
              message={message}
              disabled={revealAnswer}
              mode={mode}
              choices={choices}
            />
          </CharacterCard>
        </div>
      )}
    </div>
  );
}

export default App;