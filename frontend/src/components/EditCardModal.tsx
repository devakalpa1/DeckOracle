import { useState, useEffect } from 'react';
import { useUpdateCardMutation } from '../store/services/api';
import type { Card } from '../types';

interface EditCardModalProps {
  isOpen: boolean;
  onClose: () => void;
  card: Card | null;
  onSuccess?: () => void;
}

const EditCardModal = ({ isOpen, onClose, card, onSuccess }: EditCardModalProps) => {
  const [front, setFront] = useState('');
  const [back, setBack] = useState('');
  const [updateCard, { isLoading, error }] = useUpdateCardMutation();

  useEffect(() => {
    if (card) {
      setFront(card.front);
      setBack(card.back);
    }
  }, [card]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!card || !front.trim() || !back.trim()) {
      return;
    }

    try {
      await updateCard({
        id: card.id,
        updates: {
          front: front.trim(),
          back: back.trim(),
        }
      }).unwrap();
      
      // Close modal and call success callback
      onClose();
      if (onSuccess) {
        onSuccess();
      }
    } catch (err) {
      console.error('Failed to update card:', err);
    }
  };

  if (!isOpen || !card) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold text-primaryDark">Edit Card</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="front" className="block text-sm font-medium text-gray-700 mb-1">
              Front (Question) *
            </label>
            <textarea
              id="front"
              value={front}
              onChange={(e) => setFront(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent resize-none"
              placeholder="Enter the question or prompt..."
              rows={3}
              required
              autoFocus
            />
          </div>

          <div className="mb-4">
            <label htmlFor="back" className="block text-sm font-medium text-gray-700 mb-1">
              Back (Answer) *
            </label>
            <textarea
              id="back"
              value={back}
              onChange={(e) => setBack(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent resize-none"
              placeholder="Enter the answer..."
              rows={3}
              required
            />
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-50 border border-red-200 text-red-600 rounded-lg text-sm">
              Failed to update card. Please try again.
            </div>
          )}

          <div className="flex gap-3">
            <button
              type="button"
              onClick={onClose}
              className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
              disabled={isLoading}
            >
              Cancel
            </button>
            <button
              type="submit"
              className="flex-1 btn-primary"
              disabled={isLoading || !front.trim() || !back.trim()}
            >
              {isLoading ? 'Saving...' : 'Save Changes'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default EditCardModal;
