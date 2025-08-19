import React, { useState } from 'react';
import { Plus, Minus, Save, X } from 'lucide-react';
import { useCreateCardMutation } from '../store/services/api';

interface CardData {
  front: string;
  back: string;
}

interface BulkCardCreatorProps {
  deckId: string;
  onClose: () => void;
  onSuccess: () => void;
}

const BulkCardCreator: React.FC<BulkCardCreatorProps> = ({
  deckId,
  onClose,
  onSuccess,
}) => {
  const [cards, setCards] = useState<CardData[]>(
    Array(10).fill(null).map(() => ({ front: '', back: '' }))
  );
  const [isSaving, setIsSaving] = useState(false);
  const [createCard] = useCreateCardMutation();

  const updateCard = (index: number, field: 'front' | 'back', value: string) => {
    const newCards = [...cards];
    newCards[index] = { ...newCards[index], [field]: value };
    setCards(newCards);
  };

  const addMoreCards = (count: number) => {
    const newCards = Array(count).fill(null).map(() => ({ front: '', back: '' }));
    setCards([...cards, ...newCards]);
  };

  const removeCard = (index: number) => {
    if (cards.length > 1) {
      setCards(cards.filter((_, i) => i !== index));
    }
  };

  const handleSave = async () => {
    // Filter out empty cards
    const validCards = cards.filter(card => card.front.trim() && card.back.trim());
    
    if (validCards.length === 0) {
      alert('Please fill in at least one card');
      return;
    }

    setIsSaving(true);
    try {
      // Save all cards
      for (const card of validCards) {
        await createCard({
          deck_id: deckId,
          front: card.front,
          back: card.back,
        }).unwrap();
      }
      
      onSuccess();
      alert(`Successfully added ${validCards.length} cards!`);
    } catch (error) {
      console.error('Failed to save cards:', error);
      alert('Failed to save some cards. Please try again.');
    } finally {
      setIsSaving(false);
    }
  };

  const filledCount = cards.filter(c => c.front.trim() || c.back.trim()).length;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl w-full max-w-6xl max-h-[90vh] flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-gray-200">
          <div>
            <h2 className="text-2xl font-bold text-[rgb(18_55_64)]">
              Bulk Add Cards
            </h2>
            <p className="text-sm text-gray-600 mt-1">
              Add multiple cards at once. {filledCount} of {cards.length} cards have content.
            </p>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Cards Grid */}
        <div className="flex-1 overflow-y-auto p-6">
          <div className="space-y-4">
            {cards.map((card, index) => (
              <div
                key={index}
                className="flex gap-4 items-start p-4 bg-gray-50 rounded-lg border border-gray-200"
              >
                <div className="flex-shrink-0 w-12 h-12 bg-[rgb(84_154_171)] text-white rounded-lg flex items-center justify-center font-bold">
                  {index + 1}
                </div>
                
                <div className="flex-1 grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-xs font-medium text-gray-500 mb-1">
                      FRONT (Question/Term)
                    </label>
                    <textarea
                      value={card.front}
                      onChange={(e) => updateCard(index, 'front', e.target.value)}
                      placeholder="Enter question or term..."
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent resize-none"
                      rows={2}
                    />
                  </div>
                  
                  <div>
                    <label className="block text-xs font-medium text-gray-500 mb-1">
                      BACK (Answer/Definition)
                    </label>
                    <textarea
                      value={card.back}
                      onChange={(e) => updateCard(index, 'back', e.target.value)}
                      placeholder="Enter answer or definition..."
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent resize-none"
                      rows={2}
                    />
                  </div>
                </div>

                {cards.length > 1 && (
                  <button
                    onClick={() => removeCard(index)}
                    className="p-2 text-red-500 hover:bg-red-50 rounded-lg transition-colors"
                    title="Remove this card"
                  >
                    <Minus className="w-4 h-4" />
                  </button>
                )}
              </div>
            ))}
          </div>

          {/* Add More Cards */}
          <div className="mt-6 flex gap-2 justify-center">
            <button
              onClick={() => addMoreCards(5)}
              className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Add 5 More
            </button>
            <button
              onClick={() => addMoreCards(10)}
              className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Add 10 More
            </button>
            <button
              onClick={() => addMoreCards(20)}
              className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Add 20 More
            </button>
            <button
              onClick={() => {
                const count = prompt('How many cards to add?', '5');
                if (count && !isNaN(Number(count))) {
                  addMoreCards(Number(count));
                }
              }}
              className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors"
            >
              Custom...
            </button>
          </div>
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center p-6 border-t border-gray-200">
          <div className="text-sm text-gray-600">
            {filledCount > 0 && (
              <span className="text-green-600 font-medium">
                {filledCount} cards ready to save
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
            <button
              onClick={handleSave}
              disabled={isSaving || filledCount === 0}
              className="px-6 py-2 bg-[rgb(18_55_64)] text-white rounded-lg hover:bg-[rgb(84_154_171)] transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
            >
              {isSaving ? (
                <>Saving...</>
              ) : (
                <>
                  <Save className="w-4 h-4" />
                  Save {filledCount} Cards
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default BulkCardCreator;
