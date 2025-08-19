import React, { useState } from 'react';
import { Sparkles, X, FileText, Upload } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useCreateDeckMutation, useCreateCardMutation } from '../store/services/api';

interface GeneratedCard {
  front: string;
  back: string;
}

interface AiDeckCreatorProps {
  onClose: () => void;
}

const AiDeckCreator: React.FC<AiDeckCreatorProps> = ({ onClose }) => {
  const navigate = useNavigate();
  const [createDeck] = useCreateDeckMutation();
  const [createCard] = useCreateCardMutation();
  
  const [deckName, setDeckName] = useState('');
  const [deckDescription, setDeckDescription] = useState('');
  const [topic, setTopic] = useState('');
  const [difficulty, setDifficulty] = useState('intermediate');
  const [cardCount, setCardCount] = useState(20);
  const [uploadedFile, setUploadedFile] = useState<File | null>(null);
  const [inputMethod, setInputMethod] = useState<'text' | 'file'>('text');
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedCards, setGeneratedCards] = useState<GeneratedCard[]>([]);
  const [isCreatingDeck, setIsCreatingDeck] = useState(false);

  const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      setUploadedFile(file);
      if (!deckName) {
        setDeckName(file.name.replace(/\.[^/.]+$/, ''));
      }
    }
  };

  const handleGenerate = async () => {
    if (!topic && !uploadedFile) {
      alert('Please provide a topic or upload a file');
      return;
    }

    setIsGenerating(true);
    try {
      const formData = new FormData();
      
      if (inputMethod === 'file' && uploadedFile) {
        formData.append('file', uploadedFile);
        formData.append('card_count', cardCount.toString());
      } else {
        formData.append('topic', topic);
        formData.append('difficulty', difficulty);
        formData.append('card_count', cardCount.toString());
      }

      const response = await fetch('/api/ai/generate-deck', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Failed to generate deck');
      }

      const data = await response.json();
      
      // For now, using mock data since backend returns stubs
      const mockCards: GeneratedCard[] = Array.from({ length: cardCount }, (_, i) => ({
        front: `${topic || 'Topic'} Question ${i + 1}`,
        back: `Answer for question ${i + 1} about ${topic || 'the uploaded content'}`
      }));
      
      setGeneratedCards(mockCards);
      
      // Auto-fill deck name if not set
      if (!deckName && topic) {
        setDeckName(`${topic} Flashcards`);
      }
    } catch (error) {
      console.error('Failed to generate deck:', error);
      alert('Failed to generate deck. Please try again.');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCreateDeck = async () => {
    if (!deckName) {
      alert('Please provide a deck name');
      return;
    }

    if (generatedCards.length === 0) {
      alert('Please generate cards first');
      return;
    }

    setIsCreatingDeck(true);
    try {
      // Create the deck
      const deck = await createDeck({
        name: deckName,
        description: deckDescription || `AI-generated deck about ${topic || 'uploaded content'}`,
      }).unwrap();

      // Create all cards
      for (const card of generatedCards) {
        await createCard({
          deck_id: deck.id,
          front: card.front,
          back: card.back,
        }).unwrap();
      }

      // Navigate to the new deck
      navigate(`/decks/${deck.id}`);
    } catch (error) {
      console.error('Failed to create deck:', error);
      alert('Failed to create deck. Please try again.');
    } finally {
      setIsCreatingDeck(false);
    }
  };

  const removeCard = (index: number) => {
    setGeneratedCards(generatedCards.filter((_, i) => i !== index));
  };

  const editCard = (index: number, field: 'front' | 'back', value: string) => {
    const newCards = [...generatedCards];
    newCards[index] = { ...newCards[index], [field]: value };
    setGeneratedCards(newCards);
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-6xl max-h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-gray-200">
          <div className="flex items-center gap-3">
            <div className="p-2 bg-gradient-to-r from-purple-500 to-pink-500 rounded-lg">
              <Sparkles className="w-6 h-6 text-white" />
            </div>
            <div>
              <h2 className="text-2xl font-bold text-[rgb(18_55_64)]">
                AI-Powered Deck Creation
              </h2>
              <p className="text-sm text-gray-600 mt-1">
                Generate an entire flashcard deck with AI
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {generatedCards.length === 0 ? (
            <div className="space-y-6">
              {/* Deck Info */}
              <div className="bg-gray-50 p-6 rounded-lg border border-gray-200">
                <h3 className="text-lg font-semibold mb-4">Deck Information</h3>
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Deck Name
                    </label>
                    <input
                      type="text"
                      value={deckName}
                      onChange={(e) => setDeckName(e.target.value)}
                      placeholder="e.g., Biology Chapter 5"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Description (optional)
                    </label>
                    <textarea
                      value={deckDescription}
                      onChange={(e) => setDeckDescription(e.target.value)}
                      placeholder="Brief description of the deck..."
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent resize-none"
                      rows={2}
                    />
                  </div>
                </div>
              </div>

              {/* Input Method Tabs */}
              <div className="flex gap-2 mb-4">
                <button
                  onClick={() => setInputMethod('text')}
                  className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                    inputMethod === 'text'
                      ? 'bg-[rgb(84_154_171)] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  <FileText className="w-4 h-4" />
                  Text Input
                </button>
                <button
                  onClick={() => setInputMethod('file')}
                  className={`px-4 py-2 rounded-lg flex items-center gap-2 transition-colors ${
                    inputMethod === 'file'
                      ? 'bg-[rgb(84_154_171)] text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  <Upload className="w-4 h-4" />
                  File Upload
                </button>
              </div>

              {/* Input Content */}
              {inputMethod === 'text' ? (
                <div className="space-y-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Topic or Subject
                    </label>
                    <input
                      type="text"
                      value={topic}
                      onChange={(e) => setTopic(e.target.value)}
                      placeholder="e.g., World War II, Python Programming, Spanish Vocabulary"
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                    />
                  </div>
                  
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                      Difficulty Level
                    </label>
                    <select
                      value={difficulty}
                      onChange={(e) => setDifficulty(e.target.value)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                    >
                      <option value="beginner">Beginner</option>
                      <option value="intermediate">Intermediate</option>
                      <option value="advanced">Advanced</option>
                      <option value="expert">Expert</option>
                    </select>
                  </div>
                </div>
              ) : (
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    Upload Document
                  </label>
                  <div className="border-2 border-dashed border-gray-300 rounded-lg p-6 text-center">
                    <Upload className="w-12 h-12 text-gray-400 mx-auto mb-3" />
                    <p className="text-sm text-gray-600 mb-2">
                      Upload a document to generate flashcards from
                    </p>
                    <input
                      type="file"
                      onChange={handleFileUpload}
                      accept=".txt,.pdf,.docx,.md"
                      className="hidden"
                      id="file-upload"
                    />
                    <label
                      htmlFor="file-upload"
                      className="inline-block px-4 py-2 bg-[rgb(84_154_171)] text-white rounded-lg hover:bg-[rgb(84_154_171)]/90 cursor-pointer transition-colors"
                    >
                      Choose File
                    </label>
                    {uploadedFile && (
                      <p className="mt-3 text-sm text-green-600">
                        ✓ {uploadedFile.name}
                      </p>
                    )}
                  </div>
                </div>
              )}

              {/* Card Count */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Number of Cards to Generate
                </label>
                <div className="flex gap-2">
                  {[10, 20, 30, 50].map((count) => (
                    <button
                      key={count}
                      onClick={() => setCardCount(count)}
                      className={`px-4 py-2 rounded-lg transition-colors ${
                        cardCount === count
                          ? 'bg-[rgb(84_154_171)] text-white'
                          : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                      }`}
                    >
                      {count}
                    </button>
                  ))}
                  <input
                    type="number"
                    value={cardCount}
                    onChange={(e) => setCardCount(Number(e.target.value))}
                    min="1"
                    max="100"
                    className="w-20 px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                  />
                </div>
              </div>

              {/* Generate Button */}
              <div className="flex justify-center pt-4">
                <button
                  onClick={handleGenerate}
                  disabled={isGenerating || (!topic && !uploadedFile)}
                  className="px-8 py-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                >
                  {isGenerating ? (
                    <>Generating...</>
                  ) : (
                    <>
                      <Sparkles className="w-5 h-5" />
                      Generate Flashcards
                    </>
                  )}
                </button>
              </div>
            </div>
          ) : (
            <div className="space-y-4">
              <div className="bg-green-50 border border-green-200 rounded-lg p-4 mb-4">
                <p className="text-green-800">
                  ✓ Generated {generatedCards.length} flashcards! Review and edit them below, then create your deck.
                </p>
              </div>

              {/* Generated Cards */}
              <div className="space-y-3 max-h-96 overflow-y-auto">
                {generatedCards.map((card, index) => (
                  <div
                    key={index}
                    className="flex gap-4 items-start p-4 bg-gray-50 rounded-lg border border-gray-200"
                  >
                    <div className="flex-shrink-0 w-10 h-10 bg-[rgb(84_154_171)] text-white rounded-lg flex items-center justify-center font-bold text-sm">
                      {index + 1}
                    </div>
                    
                    <div className="flex-1 grid grid-cols-2 gap-4">
                      <div>
                        <label className="block text-xs font-medium text-gray-500 mb-1">
                          FRONT
                        </label>
                        <textarea
                          value={card.front}
                          onChange={(e) => editCard(index, 'front', e.target.value)}
                          className="w-full px-2 py-1 border border-gray-300 rounded text-sm resize-none"
                          rows={2}
                        />
                      </div>
                      
                      <div>
                        <label className="block text-xs font-medium text-gray-500 mb-1">
                          BACK
                        </label>
                        <textarea
                          value={card.back}
                          onChange={(e) => editCard(index, 'back', e.target.value)}
                          className="w-full px-2 py-1 border border-gray-300 rounded text-sm resize-none"
                          rows={2}
                        />
                      </div>
                    </div>

                    <button
                      onClick={() => removeCard(index)}
                      className="p-1 text-red-500 hover:bg-red-50 rounded transition-colors"
                      title="Remove this card"
                    >
                      <X className="w-4 h-4" />
                    </button>
                  </div>
                ))}
              </div>

              {/* Regenerate Button */}
              <div className="flex justify-center">
                <button
                  onClick={() => setGeneratedCards([])}
                  className="px-4 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
                >
                  ← Back to Generation Options
                </button>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center p-6 border-t border-gray-200">
          <div className="text-sm text-gray-600">
            {generatedCards.length > 0 && (
              <span className="text-green-600 font-medium">
                {generatedCards.length} cards ready
              </span>
            )}
          </div>
          <div className="flex gap-3">
            <button
              onClick={onClose}
              className="px-6 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
            >
              Cancel
            </button>
            {generatedCards.length > 0 && (
              <button
                onClick={handleCreateDeck}
                disabled={isCreatingDeck || !deckName}
                className="px-6 py-2 bg-[rgb(18_55_64)] text-white rounded-lg hover:bg-[rgb(84_154_171)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              >
                {isCreatingDeck ? (
                  <>Creating Deck...</>
                ) : (
                  <>Create Deck with {generatedCards.length} Cards</>
                )}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default AiDeckCreator;
