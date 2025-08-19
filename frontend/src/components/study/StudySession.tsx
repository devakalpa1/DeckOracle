import React, { useState, useEffect, useCallback } from 'react';
import { Card, Deck } from '../../types';
import { 
  useCreateStudySessionMutation, 
  useSubmitCardAnswerMutation, 
  useCompleteStudySessionMutation 
} from '../../store/services/api';

interface StudySessionProps {
  deck: Deck;
  cards: Card[];
  onComplete: () => void;
}

type CardStatus = 'easy' | 'medium' | 'hard' | 'forgot';

export default function StudySession({ deck, cards, onComplete }: StudySessionProps) {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [isFlipped, setIsFlipped] = useState(false);
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [startTime, setStartTime] = useState<number>(Date.now());
  const [cardStartTime, setCardStartTime] = useState<number>(Date.now());
  const [answers, setAnswers] = useState<Record<string, CardStatus>>({});
  
  const [createSession] = useCreateStudySessionMutation();
  const [submitAnswer] = useSubmitCardAnswerMutation();
  const [completeSession] = useCompleteStudySessionMutation();

  // Initialize study session
  useEffect(() => {
    const initSession = async () => {
      try {
        const result = await createSession({ 
          deck_id: deck.id, 
          study_mode: 'standard' 
        }).unwrap();
        setSessionId(result.id);
        setStartTime(Date.now());
      } catch (error) {
        console.error('Failed to create study session:', error);
      }
    };
    
    initSession();
  }, [deck.id, createSession]);

  const currentCard = cards[currentIndex];
  const progress = ((currentIndex + 1) / cards.length) * 100;

  const handleAnswer = useCallback(async (status: CardStatus) => {
    if (!sessionId || !currentCard) return;

    const responseTime = Date.now() - cardStartTime;
    
    try {
      await submitAnswer({
        sessionId,
        card_id: currentCard.id,
        status,
        response_time_ms: responseTime,
        is_correct: status === 'easy' || status === 'medium',
      }).unwrap();
      
      setAnswers(prev => ({ ...prev, [currentCard.id]: status }));
      
      if (currentIndex < cards.length - 1) {
        setCurrentIndex(prev => prev + 1);
        setIsFlipped(false);
        setCardStartTime(Date.now());
      } else {
        // Complete session
        const duration = Math.floor((Date.now() - startTime) / 1000);
        await completeSession(sessionId).unwrap();
        onComplete();
      }
    } catch (error) {
      console.error('Failed to submit answer:', error);
    }
  }, [sessionId, currentCard, currentIndex, cards.length, cardStartTime, startTime, submitAnswer, completeSession, onComplete]);

  const handleFlip = () => {
    setIsFlipped(prev => !prev);
  };

  const handleSkip = () => {
    handleAnswer('forgot');
  };

  const handleKeyPress = useCallback((e: KeyboardEvent) => {
    if (!isFlipped) {
      if (e.key === ' ' || e.key === 'Enter') {
        e.preventDefault();
        handleFlip();
      }
    } else {
      switch(e.key) {
        case '1':
          handleAnswer('easy');
          break;
        case '2':
          handleAnswer('medium');
          break;
        case '3':
          handleAnswer('hard');
          break;
        case '4':
          handleAnswer('forgot');
          break;
        case ' ':
        case 'Enter':
          e.preventDefault();
          handleAnswer('medium');
          break;
      }
    }
  }, [isFlipped, handleAnswer]);

  useEffect(() => {
    window.addEventListener('keydown', handleKeyPress);
    return () => window.removeEventListener('keydown', handleKeyPress);
  }, [handleKeyPress]);

  if (!currentCard) {
    return (
      <div className="study-session animate-pulse">
        {/* Progress Bar Skeleton */}
        <div className="mb-6">
          <div className="flex justify-between text-sm mb-2">
            <div className="h-4 bg-gray-200 rounded w-24"></div>
            <div className="h-4 bg-gray-200 rounded w-20"></div>
          </div>
          <div className="h-2 bg-gray-200 rounded-full"></div>
        </div>
        
        {/* Flashcard Skeleton */}
        <div className="h-96 bg-gray-200 rounded-lg mb-8"></div>
        
        {/* Button Skeletons */}
        <div className="grid grid-cols-4 gap-3">
          {[...Array(4)].map((_, i) => (
            <div key={i} className="h-14 bg-gray-200 rounded-lg"></div>
          ))}
        </div>
      </div>
    );
  }

  return (
    <div className="study-session">
      {/* Progress Bar */}
      <div className="mb-6">
        <div className="flex justify-between text-sm text-gray-600 mb-2">
          <span>Card {currentIndex + 1} of {cards.length}</span>
          <span>{Math.round(progress)}% Complete</span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-2">
          <div 
            className="bg-blue-600 h-2 rounded-full transition-all duration-300"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>

      {/* Flashcard */}
      <div className="relative">
        <div 
          className="flashcard-container cursor-pointer"
          onClick={handleFlip}
          style={{ perspective: '1000px', height: '400px' }}
        >
          <div 
            className={`flashcard relative w-full h-full transition-transform duration-500 transform-style-preserve-3d ${
              isFlipped ? 'rotate-y-180' : ''
            }`}
            style={{ 
              transformStyle: 'preserve-3d',
              transform: isFlipped ? 'rotateY(180deg)' : 'rotateY(0deg)'
            }}
          >
            {/* Front of card */}
            <div 
              className="absolute inset-0 w-full h-full bg-white rounded-lg shadow-lg p-8 flex items-center justify-center backface-hidden"
              style={{ backfaceVisibility: 'hidden' }}
            >
              <div className="text-center">
                <p className="text-2xl font-medium text-gray-900">{currentCard.front}</p>
                <p className="mt-4 text-sm text-gray-500">Click or press Space to reveal answer</p>
              </div>
            </div>

            {/* Back of card */}
            <div 
              className="absolute inset-0 w-full h-full bg-white rounded-lg shadow-lg p-8 flex items-center justify-center backface-hidden"
              style={{ 
                backfaceVisibility: 'hidden',
                transform: 'rotateY(180deg)'
              }}
            >
              <div className="text-center">
                <p className="text-2xl font-medium text-gray-900">{currentCard.back}</p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Answer buttons */}
      {isFlipped && (
        <div className="mt-8 space-y-4">
          <div className="text-center text-sm text-gray-600 mb-2">
            How well did you know this?
          </div>
          <div className="grid grid-cols-4 gap-3">
            <button
              onClick={() => handleAnswer('easy')}
              className="px-4 py-3 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors"
            >
              <div className="font-medium">Easy</div>
              <div className="text-xs opacity-75">Press 1</div>
            </button>
            <button
              onClick={() => handleAnswer('medium')}
              className="px-4 py-3 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
            >
              <div className="font-medium">Good</div>
              <div className="text-xs opacity-75">Press 2</div>
            </button>
            <button
              onClick={() => handleAnswer('hard')}
              className="px-4 py-3 bg-yellow-500 text-white rounded-lg hover:bg-yellow-600 transition-colors"
            >
              <div className="font-medium">Hard</div>
              <div className="text-xs opacity-75">Press 3</div>
            </button>
            <button
              onClick={() => handleAnswer('forgot')}
              className="px-4 py-3 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
            >
              <div className="font-medium">Again</div>
              <div className="text-xs opacity-75">Press 4</div>
            </button>
          </div>
        </div>
      )}

      {/* Skip button */}
      {!isFlipped && (
        <div className="mt-8 flex justify-center">
          <button
            onClick={handleSkip}
            className="px-6 py-2 text-gray-600 hover:text-gray-800 transition-colors"
          >
            Skip Card
          </button>
        </div>
      )}
    </div>
  );
}
