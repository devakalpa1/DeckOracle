import { Link } from 'react-router-dom';
import { useState } from 'react';
import { Plus, Sparkles } from 'lucide-react';
import { useGetDecksQuery } from '../store/services/api';
import CreateDeckModal from '../components/CreateDeckModal';
import AiDeckCreator from '../components/AiDeckCreator';

const DeckListPage = () => {
  const { data: decks, isLoading, error } = useGetDecksQuery();
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isAiCreatorOpen, setIsAiCreatorOpen] = useState(false);

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">Loading decks...</div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center text-red-500">
          Failed to load decks. Please try again later.
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold text-primaryDark">My Decks</h1>
        <div className="flex gap-2">
          <button 
            onClick={() => setIsAiCreatorOpen(true)}
            className="px-4 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all flex items-center gap-2"
          >
            <Sparkles className="w-4 h-4" />
            AI Generate Deck
          </button>
          <button 
            onClick={() => setIsCreateModalOpen(true)}
            className="btn-primary flex items-center gap-2"
          >
            <Plus className="w-4 h-4" />
            Create New Deck
          </button>
        </div>
      </div>

      {decks && decks.length > 0 ? (
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {decks.map((deck) => (
            <Link
              key={deck.id}
              to={`/decks/${deck.id}`}
              className="card hover:shadow-lg transition-shadow"
            >
              <h3 className="text-xl font-semibold mb-2">{deck.name}</h3>
              <p className="text-gray-600 mb-4">
                {deck.description || 'No description'}
              </p>
              <div className="flex justify-between text-sm text-gray-500">
                <span>{deck.cardCount || 0} cards</span>
                <span>{deck.isPublic ? 'Public' : 'Private'}</span>
              </div>
            </Link>
          ))}
        </div>
      ) : (
        <div className="text-center py-12">
          <div className="text-6xl mb-4">📚</div>
          <h2 className="text-2xl font-semibold mb-2">No decks yet</h2>
          <p className="text-gray-600 mb-6">
            Create your first deck to start learning!
          </p>
          <div className="flex gap-3 justify-center">
            <button 
              onClick={() => setIsAiCreatorOpen(true)}
              className="px-6 py-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all flex items-center gap-2"
            >
              <Sparkles className="w-5 h-5" />
              Generate with AI
            </button>
            <button 
              onClick={() => setIsCreateModalOpen(true)}
              className="btn-primary px-6 py-3 flex items-center gap-2"
            >
              <Plus className="w-5 h-5" />
              Create Manually
            </button>
          </div>
        </div>
      )}

      <CreateDeckModal
        isOpen={isCreateModalOpen}
        onClose={() => setIsCreateModalOpen(false)}
        onSuccess={() => {
          // The mutation will automatically refetch the decks list
          // due to the invalidatesTags in the API configuration
        }}
      />

      {isAiCreatorOpen && (
        <AiDeckCreator
          onClose={() => setIsAiCreatorOpen(false)}
        />
      )}
    </div>
  );
};

export default DeckListPage;
