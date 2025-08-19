import { useParams, Link, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { Upload, Download, Sparkles, Plus } from 'lucide-react';
import { useGetDeckQuery, useGetCardsQuery, useDeleteCardMutation, useDeleteDeckMutation } from '../store/services/api';
import BulkCardCreator from '../components/BulkCardCreator';
import EditCardModal from '../components/EditCardModal';
import EditDeckModal from '../components/EditDeckModal';
import ImportExportModal from '../components/ImportExportModal';
import AiCardGenerator from '../components/AiCardGenerator';
import type { Card } from '../types';

const DeckViewPage = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data: deck, isLoading: deckLoading } = useGetDeckQuery(id!);
  const { data: cards, isLoading: cardsLoading } = useGetCardsQuery(id!);
  const [deleteCard] = useDeleteCardMutation();
  const [deleteDeck] = useDeleteDeckMutation();
  
  // Modal states
  const [isBulkCardCreatorOpen, setIsBulkCardCreatorOpen] = useState(false);
  const [isEditCardModalOpen, setIsEditCardModalOpen] = useState(false);
  const [isEditDeckModalOpen, setIsEditDeckModalOpen] = useState(false);
  const [isImportExportModalOpen, setIsImportExportModalOpen] = useState(false);
  const [isAiGeneratorOpen, setIsAiGeneratorOpen] = useState(false);
  const [selectedCard, setSelectedCard] = useState<Card | null>(null);
  
  const handleEditCard = (card: Card) => {
    setSelectedCard(card);
    setIsEditCardModalOpen(true);
  };
  
  const handleDeleteCard = async (cardId: string) => {
    if (window.confirm('Are you sure you want to delete this card?')) {
      try {
        await deleteCard(cardId).unwrap();
      } catch (error) {
        console.error('Failed to delete card:', error);
        alert('Failed to delete card. Please try again.');
      }
    }
  };
  
  const handleDeleteDeck = async () => {
    if (window.confirm('Are you sure you want to delete this deck? This will delete all cards in the deck.')) {
      try {
        await deleteDeck(id!).unwrap();
        navigate('/decks');
      } catch (error) {
        console.error('Failed to delete deck:', error);
        alert('Failed to delete deck. Please try again.');
      }
    }
  };

  if (deckLoading || cardsLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">Loading deck...</div>
      </div>
    );
  }

  if (!deck) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="text-center">
          <h2 className="text-2xl font-semibold mb-4">Deck not found</h2>
          <Link to="/decks" className="text-primary hover:underline">
            Back to decks
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-4xl mx-auto">
        <div className="flex justify-between items-start mb-8">
          <div>
            <h1 className="text-3xl font-bold text-primaryDark mb-2">{deck.name}</h1>
            <p className="text-gray-600">{deck.description}</p>
          </div>
          <div className="flex gap-2 flex-wrap">
            <Link
              to={`/study/${deck.id}`}
              className="btn-primary"
            >
              Study Now
            </Link>
            <button 
              onClick={() => setIsEditDeckModalOpen(true)}
              className="btn-secondary"
            >
              Edit Deck
            </button>
            <button
              onClick={() => setIsImportExportModalOpen(true)}
              className="px-4 py-2 border border-[rgb(84_154_171)] text-[rgb(84_154_171)] rounded-lg hover:bg-[rgb(84_154_171)]/10 transition-colors flex items-center gap-2"
            >
              <Upload className="w-4 h-4" />
              Import/Export
            </button>
            <button 
              onClick={handleDeleteDeck}
              className="px-4 py-2 border border-red-300 text-red-600 rounded-lg hover:bg-red-50 transition-colors"
            >
              Delete Deck
            </button>
          </div>
        </div>

        <div className="grid md:grid-cols-3 gap-4 mb-8">
          <div className="card">
            <div className="text-sm text-gray-600">Total Cards</div>
            <div className="text-2xl font-bold text-primary">{cards?.length || 0}</div>
          </div>
          <div className="card">
            <div className="text-sm text-gray-600">Cards Due</div>
            <div className="text-2xl font-bold text-accent">0</div>
          </div>
          <div className="card">
            <div className="text-sm text-gray-600">Last Studied</div>
            <div className="text-2xl font-bold text-primaryDark">
              {deck.lastStudied ? new Date(deck.lastStudied).toLocaleDateString() : 'Never'}
            </div>
          </div>
        </div>

        <div className="mb-4 flex justify-between items-center">
          <h2 className="text-2xl font-semibold">Cards</h2>
          <div className="flex gap-2">
            <button 
              onClick={() => setIsAiGeneratorOpen(true)}
              className="px-4 py-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white rounded-lg hover:from-purple-600 hover:to-pink-600 transition-all flex items-center gap-2"
            >
              <Sparkles className="w-4 h-4" />
              AI Generate
            </button>
            <button 
              onClick={() => setIsBulkCardCreatorOpen(true)}
              className="btn-primary flex items-center gap-2"
            >
              <Plus className="w-4 h-4" />
              Bulk Add Cards
            </button>
          </div>
        </div>

        {cards && cards.length > 0 ? (
          <div className="space-y-4">
            {cards.map((card) => (
              <div key={card.id} className="card">
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <div className="font-medium mb-2">{card.front}</div>
                    <div className="text-gray-600">{card.back}</div>
                  </div>
                  <div className="flex gap-2 ml-4">
                    <button 
                      onClick={() => handleEditCard(card)}
                      className="text-primary hover:text-primaryDark"
                    >
                      Edit
                    </button>
                    <button 
                      onClick={() => handleDeleteCard(card.id)}
                      className="text-red-500 hover:text-red-700"
                    >
                      Delete
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="card text-center py-12">
            <div className="text-4xl mb-4">🎴</div>
            <h3 className="text-xl font-semibold mb-2">No cards yet</h3>
            <p className="text-gray-600 mb-4">
              Add cards to start studying this deck
            </p>
            <button 
              onClick={() => setIsBulkCardCreatorOpen(true)}
              className="btn-primary"
            >
              Add Cards
            </button>
          </div>
        )}
      </div>
      
      {/* Modals */}
      {isBulkCardCreatorOpen && (
        <BulkCardCreator
          deckId={id!}
          onClose={() => setIsBulkCardCreatorOpen(false)}
          onSuccess={() => {
            setIsBulkCardCreatorOpen(false);
            // Trigger refetch of cards
            window.location.reload();
          }}
        />
      )}
      
      <EditCardModal
        isOpen={isEditCardModalOpen}
        onClose={() => {
          setIsEditCardModalOpen(false);
          setSelectedCard(null);
        }}
        card={selectedCard}
      />
      
      <EditDeckModal
        isOpen={isEditDeckModalOpen}
        onClose={() => setIsEditDeckModalOpen(false)}
        deck={deck || null}
      />
      
      {/* Import/Export Modal */}
      <ImportExportModal
        isOpen={isImportExportModalOpen}
        onClose={() => setIsImportExportModalOpen(false)}
        deckId={id!}
        deckName={deck?.name}
      />

      {/* AI Generator Modal */}
      {isAiGeneratorOpen && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
          <div className="bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto p-6">
            <AiCardGenerator
              deckId={id!}
              onClose={() => setIsAiGeneratorOpen(false)}
              onSuccess={() => {
                setIsAiGeneratorOpen(false);
                // Trigger refetch of cards
                window.location.reload(); // Simple refresh for now
              }}
            />
          </div>
        </div>
      )}
    </div>
  );
};

export default DeckViewPage;
