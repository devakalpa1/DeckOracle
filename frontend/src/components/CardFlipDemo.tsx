import { useState } from 'react';
import { motion } from 'framer-motion';
import { DndProvider, useDrag, useDrop } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';

interface CardData {
  id: string;
  front: string;
  back: string;
}

const FlippableCard = ({ card }: { card: CardData }) => {
  const [isFlipped, setIsFlipped] = useState(false);

  return (
    <div style={{ perspective: '1000px' }}>
      <motion.div
        className="relative w-64 h-40 cursor-pointer"
        onClick={() => setIsFlipped(!isFlipped)}
        animate={{ rotateY: isFlipped ? 180 : 0 }}
        transition={{ duration: 0.6 }}
        style={{ transformStyle: 'preserve-3d' }}
      >
        {/* Front */}
        <div 
          className="absolute inset-0 bg-gradient-to-br from-[rgb(84_154_171)] to-[rgb(18_55_64)] rounded-lg shadow-lg flex items-center justify-center text-white p-4"
          style={{ backfaceVisibility: 'hidden' }}
        >
          <div className="text-center">
            <div className="text-xs uppercase tracking-wide mb-2">Question</div>
            <div className="font-medium">{card.front}</div>
          </div>
        </div>

        {/* Back */}
        <div 
          className="absolute inset-0 bg-gradient-to-br from-[rgb(241_128_45)] to-[rgb(241_128_45)]/80 rounded-lg shadow-lg flex items-center justify-center text-white p-4"
          style={{ 
            backfaceVisibility: 'hidden',
            transform: 'rotateY(180deg)'
          }}
        >
          <div className="text-center">
            <div className="text-xs uppercase tracking-wide mb-2">Answer</div>
            <div className="font-medium">{card.back}</div>
          </div>
        </div>
      </motion.div>
    </div>
  );
};

const DraggableCard = ({ card, index, moveCard }: { 
  card: CardData; 
  index: number; 
  moveCard: (dragIndex: number, hoverIndex: number) => void;
}) => {
  const [{ isDragging }, drag] = useDrag({
    type: 'card',
    item: { index },
    collect: (monitor) => ({
      isDragging: monitor.isDragging(),
    }),
  });

  const [, drop] = useDrop({
    accept: 'card',
    hover: (item: { index: number }) => {
      if (item.index !== index) {
        moveCard(item.index, index);
        item.index = index;
      }
    },
  });

  return (
    <div
      ref={(node) => drag(drop(node))}
      className={`p-4 bg-white rounded-lg shadow-md border border-[rgb(176_215_225)]/30 cursor-move transition-all hover:shadow-lg ${
        isDragging ? 'opacity-50' : 'opacity-100'
      }`}
    >
      <div className="font-medium text-[rgb(18_55_64)]">{card.front}</div>
      <div className="text-sm text-gray-600 mt-1">{card.back}</div>
    </div>
  );
};

const CardFlipDemo = () => {
  const [cards, setCards] = useState<CardData[]>([
    { id: '1', front: 'What is React?', back: 'A JavaScript library for building UIs' },
    { id: '2', front: 'What is Redux?', back: 'A state management library' },
    { id: '3', front: 'What is TypeScript?', back: 'JavaScript with static typing' },
  ]);

  const moveCard = (dragIndex: number, hoverIndex: number) => {
    const draggedCard = cards[dragIndex];
    const newCards = [...cards];
    newCards.splice(dragIndex, 1);
    newCards.splice(hoverIndex, 0, draggedCard);
    setCards(newCards);
  };

  return (
    <div className="p-8 bg-white rounded-lg shadow-sm border border-[rgb(176_215_225)]/30">
      <h2 className="text-2xl font-bold text-[rgb(18_55_64)] mb-6">Interactive Demo</h2>
      
      <div className="mb-12">
        <h3 className="text-lg font-semibold mb-4 text-[rgb(18_55_64)]">Card Flip Animation</h3>
        <p className="text-gray-600 mb-6">Click the cards to flip them!</p>
        <div className="flex gap-6 flex-wrap">
          {cards.map((card) => (
            <FlippableCard key={card.id} card={card} />
          ))}
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold mb-4 text-[rgb(18_55_64)]">Drag & Drop Cards</h3>
        <p className="text-gray-600 mb-6">Drag cards to reorder them!</p>
        <DndProvider backend={HTML5Backend}>
          <div className="space-y-3 max-w-md">
            {cards.map((card, index) => (
              <DraggableCard
                key={card.id}
                card={card}
                index={index}
                moveCard={moveCard}
              />
            ))}
          </div>
        </DndProvider>
      </div>
    </div>
  );
};

export default CardFlipDemo;
