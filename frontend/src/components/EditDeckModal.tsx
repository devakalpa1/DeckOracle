import { useState, useEffect } from 'react';
import { useUpdateDeckMutation } from '../store/services/api';
import type { Deck } from '../types';

interface EditDeckModalProps {
  isOpen: boolean;
  onClose: () => void;
  deck: Deck | null;
  onSuccess?: () => void;
}

const EditDeckModal = ({ isOpen, onClose, deck, onSuccess }: EditDeckModalProps) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [isPublic, setIsPublic] = useState(false);
  const [updateDeck, { isLoading, error }] = useUpdateDeckMutation();

  useEffect(() => {
    if (deck) {
      setName(deck.name);
      setDescription(deck.description || '');
      setIsPublic(deck.isPublic);
    }
  }, [deck]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!deck || !name.trim()) {
      return;
    }

    try {
      await updateDeck({
        id: deck.id,
        updates: {
          name: name.trim(),
          description: description.trim() || undefined,
          isPublic,
        }
      }).unwrap();
      
      // Close modal and call success callback
      onClose();
      if (onSuccess) {
        onSuccess();
      }
    } catch (err) {
      console.error('Failed to update deck:', err);
    }
  };

  if (!isOpen || !deck) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white rounded-lg p-6 w-full max-w-md mx-4">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-bold text-primaryDark">Edit Deck</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
              Deck Name *
            </label>
            <input
              type="text"
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent"
              placeholder="e.g., Spanish Vocabulary"
              required
              autoFocus
            />
          </div>

          <div className="mb-4">
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">
              Description
            </label>
            <textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent resize-none"
              placeholder="Add a description for your deck..."
              rows={3}
            />
          </div>

          <div className="mb-6">
            <label className="flex items-center space-x-2 cursor-pointer">
              <input
                type="checkbox"
                checked={isPublic}
                onChange={(e) => setIsPublic(e.target.checked)}
                className="w-4 h-4 text-accent rounded focus:ring-accent"
              />
              <span className="text-sm font-medium text-gray-700">
                Make this deck public
              </span>
            </label>
            <p className="text-xs text-gray-500 mt-1 ml-6">
              Public decks can be viewed and used by other users
            </p>
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-50 border border-red-200 text-red-600 rounded-lg text-sm">
              Failed to update deck. Please try again.
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
              disabled={isLoading || !name.trim()}
            >
              {isLoading ? 'Saving...' : 'Save Changes'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default EditDeckModal;
