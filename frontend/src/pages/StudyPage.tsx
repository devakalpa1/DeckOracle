import { useState } from 'react';
import { useParams, Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import { useGetDeckQuery, useGetCardsQuery } from '../store/services/api';

const StudyPage = () => {
  const { deckId } = useParams<{ deckId: string }>();
  const { data: deck } = useGetDeckQuery(deckId!);
  const { data: cards, isLoading } = useGetCardsQuery(deckId!);
  const [currentCardIndex, setCurrentCardIndex] = useState(0);
  const [isFlipped, setIsFlipped] = useState(false);
  const [studyStats, setStudyStats] = useState({
    correct: 0,
    wrong: 0,
    skipped: 0,
  });

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">Loading cards...</div>
      </div>
    );
  }

  if (!cards || cards.length === 0) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h2 className="text-2xl font-semibold mb-4">No cards to study</h2>
            <Link to={`/decks/${deckId}`} className="text-primaryDark hover:text-primary hover:underline">
            Back to deck
          </Link>
        </div>
      </div>
    );
  }

  const currentCard = cards[currentCardIndex];
  const progress = ((currentCardIndex + 1) / cards.length) * 100;

  const handleAnswer = (answer: 'correct' | 'wrong' | 'skip') => {
    setStudyStats(prev => ({
      ...prev,
      [answer === 'skip' ? 'skipped' : answer]: prev[answer === 'skip' ? 'skipped' : answer] + 1,
    }));

    if (currentCardIndex < cards.length - 1) {
      setCurrentCardIndex(prev => prev + 1);
      setIsFlipped(false);
    }
  };

  const handleFlip = () => {
    setIsFlipped(!isFlipped);
  };

  if (currentCardIndex >= cards.length) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="max-w-2xl mx-auto text-center">
          <h1 className="text-3xl font-bold mb-8">Study Session Complete!</h1>
          <div className="card">
            <h2 className="text-xl font-semibold mb-4">Your Results</h2>
            <div className="grid grid-cols-3 gap-4 mb-6">
              <div>
                <div className="text-3xl font-bold text-green-500">{studyStats.correct}</div>
                <div className="text-gray-600">Correct</div>
              </div>
              <div>
                <div className="text-3xl font-bold text-red-500">{studyStats.wrong}</div>
                <div className="text-gray-600">Wrong</div>
              </div>
              <div>
                <div className="text-3xl font-bold text-gray-500">{studyStats.skipped}</div>
                <div className="text-gray-600">Skipped</div>
              </div>
            </div>
            <div className="flex gap-4 justify-center">
              <button 
                onClick={() => {
                  setCurrentCardIndex(0);
                  setStudyStats({ correct: 0, wrong: 0, skipped: 0 });
                  setIsFlipped(false);
                }}
                className="btn-primary"
              >
                Study Again
              </button>
              <Link to={`/decks/${deckId}`} className="btn-secondary inline-block">
                Back to Deck
              </Link>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-2xl mx-auto">
        <div className="mb-6">
          <div className="flex justify-between items-center mb-2">
            <h1 className="text-2xl font-bold">{deck?.name}</h1>
            <Link to={`/decks/${deckId}`} className="text-gray-600 hover:text-primaryDark">
              Exit Study
            </Link>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div 
              className="bg-primary h-2 rounded-full transition-all duration-300"
              style={{ width: `${progress}%` }}
            />
          </div>
          <div className="text-sm text-gray-600 mt-1">
            Card {currentCardIndex + 1} of {cards.length}
          </div>
        </div>

        <div className="mb-8" style={{ perspective: '1000px' }}>
          <motion.div
            className="relative h-80 cursor-pointer"
            onClick={handleFlip}
            animate={{ rotateY: isFlipped ? 180 : 0 }}
            transition={{ duration: 0.6 }}
            style={{ transformStyle: 'preserve-3d' }}
          >
            {/* Front of card */}
            <div 
              className="absolute inset-0 card flex items-center justify-center text-center backface-hidden"
              style={{ backfaceVisibility: 'hidden' }}
            >
              <div>
                <div className="text-sm text-gray-500 mb-2">Question</div>
                <div className="text-xl">{currentCard.front}</div>
              </div>
            </div>

            {/* Back of card */}
            <div 
              className="absolute inset-0 card flex items-center justify-center text-center backface-hidden"
              style={{ 
                backfaceVisibility: 'hidden',
                transform: 'rotateY(180deg)'
              }}
            >
              <div>
                <div className="text-sm text-gray-500 mb-2">Answer</div>
                <div className="text-xl">{currentCard.back}</div>
              </div>
            </div>
          </motion.div>
        </div>

        <div className="text-center mb-4">
          {!isFlipped ? (
            <p className="text-gray-600">Click the card to reveal the answer</p>
          ) : (
            <p className="text-gray-600">How did you do?</p>
          )}
        </div>

        {isFlipped && (
          <div className="flex gap-4 justify-center">
            <button
              onClick={() => handleAnswer('wrong')}
              className="px-6 py-3 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
            >
              Wrong
            </button>
            <button
              onClick={() => handleAnswer('skip')}
              className="px-6 py-3 bg-gray-500 text-white rounded-lg hover:bg-gray-600 transition-colors"
            >
              Skip
            </button>
            <button
              onClick={() => handleAnswer('correct')}
              className="px-6 py-3 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
            >
              Correct
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default StudyPage;
