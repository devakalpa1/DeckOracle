import React, { useState } from 'react';
import { Sparkles, Upload, FileText, Brain, Loader2, AlertCircle, CheckCircle } from 'lucide-react';
import { useCreateCardMutation } from '../store/services/api';

interface AiCardGeneratorProps {
  deckId: string;
  onClose: () => void;
  onSuccess: () => void;
}

interface GeneratedCard {
  front: string;
  back: string;
  explanation?: string;
  difficulty?: number;
  selected: boolean;
}

const AiCardGenerator: React.FC<AiCardGeneratorProps> = ({
  deckId,
  onClose,
  onSuccess,
}) => {
  const [activeTab, setActiveTab] = useState<'text' | 'file'>('text');
  const [inputText, setInputText] = useState('');
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedCards, setGeneratedCards] = useState<GeneratedCard[]>([]);
  const [generationOptions, setGenerationOptions] = useState({
    maxCards: 10,
    difficulty: 'medium',
    includeExplanations: false,
    cardFormat: 'question_answer',
  });
  const [error, setError] = useState<string | null>(null);

  const [createCard] = useCreateCardMutation();

  const handleGenerate = async () => {
    setError(null);
    setIsGenerating(true);

    try {
      // Call AI generation endpoint
      const response = await fetch('/api/v1/ai/generate-cards', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${localStorage.getItem('token')}`,
        },
        body: JSON.stringify({
          deck_id: deckId,
          content_type: activeTab,
          content: activeTab === 'text' ? inputText : undefined,
          file: activeTab === 'file' ? selectedFile?.name : undefined,
          options: generationOptions,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to generate cards');
      }

      const data = await response.json();
      
      // For now, create mock data if AI isn't fully implemented
      const mockCards: GeneratedCard[] = [
        {
          front: "What is the capital of France?",
          back: "Paris",
          explanation: "Paris has been the capital of France since 987 CE",
          difficulty: 2,
          selected: true,
        },
        {
          front: "What is photosynthesis?",
          back: "The process by which plants convert light energy into chemical energy",
          explanation: "This process occurs in chloroplasts using chlorophyll",
          difficulty: 3,
          selected: true,
        },
        {
          front: "What year did World War II end?",
          back: "1945",
          explanation: "The war ended with Japan's surrender on September 2, 1945",
          difficulty: 2,
          selected: true,
        },
      ];

      setGeneratedCards(data.cards || mockCards);
    } catch (err: any) {
      setError(err.message || 'Failed to generate cards');
      // Use mock data for demonstration
      const mockCards: GeneratedCard[] = Array.from({ length: generationOptions.maxCards }, (_, i) => ({
        front: `Sample Question ${i + 1}: Generated from your ${activeTab === 'text' ? 'text' : 'file'}`,
        back: `Sample Answer ${i + 1}: This would be the AI-generated answer based on your content`,
        explanation: generationOptions.includeExplanations ? `Explanation ${i + 1}: Additional context would appear here` : undefined,
        difficulty: Math.floor(Math.random() * 5) + 1,
        selected: true,
      }));
      setGeneratedCards(mockCards);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleSaveCards = async () => {
    const selectedCards = generatedCards.filter(card => card.selected);
    
    try {
      for (const card of selectedCards) {
        await createCard({
          deck_id: deckId,
          front: card.front,
          back: card.back + (card.explanation ? `\n\n📝 ${card.explanation}` : ''),
        }).unwrap();
      }
      onSuccess();
    } catch (err) {
      setError('Failed to save cards');
    }
  };

  const toggleCardSelection = (index: number) => {
    setGeneratedCards(prev => prev.map((card, i) => 
      i === index ? { ...card, selected: !card.selected } : card
    ));
  };

  const selectedCount = generatedCards.filter(c => c.selected).length;

  return (
    <div className="max-w-4xl mx-auto">
      {/* Header */}
      <div className="flex items-center gap-3 mb-6">
        <div className="p-3 bg-gradient-to-br from-purple-500 to-pink-500 rounded-lg">
          <Sparkles className="w-6 h-6 text-white" />
        </div>
        <div>
          <h2 className="text-2xl font-bold text-[rgb(18_55_64)]">AI Card Generator</h2>
          <p className="text-gray-600">Generate flashcards automatically from your content</p>
        </div>
      </div>

      {!generatedCards.length ? (
        <>
          {/* Input Tabs */}
          <div className="flex gap-2 mb-6">
            <button
              className={`flex-1 py-3 px-4 rounded-lg font-medium transition-all ${
                activeTab === 'text'
                  ? 'bg-[rgb(84_154_171)] text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
              onClick={() => setActiveTab('text')}
            >
              <FileText className="w-4 h-4 inline mr-2" />
              From Text
            </button>
            <button
              className={`flex-1 py-3 px-4 rounded-lg font-medium transition-all ${
                activeTab === 'file'
                  ? 'bg-[rgb(84_154_171)] text-white'
                  : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
              }`}
              onClick={() => setActiveTab('file')}
            >
              <Upload className="w-4 h-4 inline mr-2" />
              From File
            </button>
          </div>

          {/* Input Area */}
          {activeTab === 'text' ? (
            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Enter or paste your content
              </label>
              <textarea
                value={inputText}
                onChange={(e) => setInputText(e.target.value)}
                className="w-full h-64 p-4 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="Paste your study material, article, or notes here. The AI will generate flashcards based on the key concepts..."
              />
            </div>
          ) : (
            <div className="mb-6">
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Upload a document
              </label>
              <div className="border-2 border-dashed border-gray-300 rounded-lg p-8 text-center hover:border-[rgb(84_154_171)] transition-colors">
                <input
                  type="file"
                  accept=".pdf,.docx,.txt,.md"
                  onChange={(e) => setSelectedFile(e.target.files?.[0] || null)}
                  className="hidden"
                  id="file-upload"
                />
                <label htmlFor="file-upload" className="cursor-pointer">
                  <Upload className="w-12 h-12 mx-auto text-gray-400 mb-3" />
                  {selectedFile ? (
                    <div>
                      <p className="text-sm font-medium text-gray-900">{selectedFile.name}</p>
                      <p className="text-xs text-gray-500">{(selectedFile.size / 1024).toFixed(2)} KB</p>
                    </div>
                  ) : (
                    <>
                      <p className="text-sm font-medium text-gray-900">Click to upload or drag and drop</p>
                      <p className="text-xs text-gray-500 mt-1">PDF, DOCX, TXT, MD up to 10MB</p>
                    </>
                  )}
                </label>
              </div>
            </div>
          )}

          {/* Generation Options */}
          <div className="bg-gray-50 rounded-lg p-6 mb-6">
            <h3 className="text-lg font-semibold mb-4">Generation Options</h3>
            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Number of Cards
                </label>
                <input
                  type="number"
                  min="1"
                  max="50"
                  value={generationOptions.maxCards}
                  onChange={(e) => setGenerationOptions(prev => ({
                    ...prev,
                    maxCards: parseInt(e.target.value) || 10
                  }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)]"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Difficulty Level
                </label>
                <select
                  value={generationOptions.difficulty}
                  onChange={(e) => setGenerationOptions(prev => ({
                    ...prev,
                    difficulty: e.target.value
                  }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)]"
                >
                  <option value="easy">Easy</option>
                  <option value="medium">Medium</option>
                  <option value="hard">Hard</option>
                  <option value="mixed">Mixed</option>
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Card Format
                </label>
                <select
                  value={generationOptions.cardFormat}
                  onChange={(e) => setGenerationOptions(prev => ({
                    ...prev,
                    cardFormat: e.target.value
                  }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)]"
                >
                  <option value="question_answer">Question & Answer</option>
                  <option value="term_definition">Term & Definition</option>
                  <option value="concept_explanation">Concept & Explanation</option>
                </select>
              </div>
              <div className="flex items-center">
                <label className="flex items-center cursor-pointer">
                  <input
                    type="checkbox"
                    checked={generationOptions.includeExplanations}
                    onChange={(e) => setGenerationOptions(prev => ({
                      ...prev,
                      includeExplanations: e.target.checked
                    }))}
                    className="mr-2"
                  />
                  <span className="text-sm font-medium text-gray-700">Include explanations</span>
                </label>
              </div>
            </div>
          </div>

          {/* Error Display */}
          {error && (
            <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-lg flex items-start gap-3">
              <AlertCircle className="w-5 h-5 text-red-600 flex-shrink-0 mt-0.5" />
              <div>
                <p className="text-sm text-red-800">{error}</p>
                <p className="text-xs text-red-600 mt-1">Using sample data for demonstration</p>
              </div>
            </div>
          )}

          {/* Generate Button */}
          <div className="flex justify-end gap-3">
            <button
              onClick={onClose}
              className="px-6 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={handleGenerate}
              disabled={isGenerating || (!inputText && !selectedFile)}
              className="px-6 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            >
              {isGenerating ? (
                <>
                  <Loader2 className="w-4 h-4 animate-spin" />
                  Generating...
                </>
              ) : (
                <>
                  <Brain className="w-4 h-4" />
                  Generate Cards
                </>
              )}
            </button>
          </div>
        </>
      ) : (
        <>
          {/* Generated Cards Review */}
          <div className="mb-6">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-semibold">Generated Cards ({selectedCount}/{generatedCards.length} selected)</h3>
              <button
                onClick={() => setGeneratedCards([])}
                className="text-sm text-gray-600 hover:text-gray-800"
              >
                Generate New
              </button>
            </div>
            
            <div className="space-y-3 max-h-96 overflow-y-auto">
              {generatedCards.map((card, index) => (
                <div
                  key={index}
                  className={`p-4 border rounded-lg cursor-pointer transition-all ${
                    card.selected
                      ? 'border-[rgb(84_154_171)] bg-blue-50'
                      : 'border-gray-200 bg-white hover:bg-gray-50'
                  }`}
                  onClick={() => toggleCardSelection(index)}
                >
                  <div className="flex items-start gap-3">
                    <input
                      type="checkbox"
                      checked={card.selected}
                      onChange={() => toggleCardSelection(index)}
                      className="mt-1"
                      onClick={(e) => e.stopPropagation()}
                    />
                    <div className="flex-1">
                      <div className="mb-2">
                        <span className="text-xs font-medium text-gray-500">Front:</span>
                        <p className="text-sm font-medium">{card.front}</p>
                      </div>
                      <div className="mb-2">
                        <span className="text-xs font-medium text-gray-500">Back:</span>
                        <p className="text-sm">{card.back}</p>
                      </div>
                      {card.explanation && (
                        <div className="mt-2 p-2 bg-gray-100 rounded text-xs text-gray-600">
                          📝 {card.explanation}
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Action Buttons */}
          <div className="flex justify-between">
            <button
              onClick={() => setGeneratedCards(prev => prev.map(c => ({ ...c, selected: !selectedCount || selectedCount < generatedCards.length })))}
              className="text-sm text-[rgb(84_154_171)] hover:text-[rgb(18_55_64)]"
            >
              {selectedCount === generatedCards.length ? 'Deselect All' : 'Select All'}
            </button>
            <div className="flex gap-3">
              <button
                onClick={() => setGeneratedCards([])}
                className="px-6 py-2 text-gray-700 hover:bg-gray-100 rounded-lg transition-colors"
              >
                Back
              </button>
              <button
                onClick={handleSaveCards}
                disabled={selectedCount === 0}
                className="px-6 py-2 bg-[rgb(18_55_64)] text-white rounded-lg hover:bg-[rgb(84_154_171)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
              >
                <CheckCircle className="w-4 h-4" />
                Save {selectedCount} Cards
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default AiCardGenerator;
