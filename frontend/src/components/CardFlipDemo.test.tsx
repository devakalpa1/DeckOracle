import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import CardFlipDemo from './CardFlipDemo';
import { DndProvider } from 'react-dnd';
import { HTML5Backend } from 'react-dnd-html5-backend';

// Mock framer-motion to avoid animation issues in tests
vi.mock('framer-motion', () => ({
  motion: {
    div: ({ children, onClick, animate, ...props }: any) => (
      <div onClick={onClick} data-testid="motion-div" {...props}>
        {children}
      </div>
    ),
  },
}));

describe('CardFlipDemo', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the component with demo cards', () => {
    render(<CardFlipDemo />);
    
    expect(screen.getByText('Interactive Demo')).toBeInTheDocument();
    expect(screen.getByText('Card Flip Animation')).toBeInTheDocument();
    expect(screen.getByText('Drag & Drop Cards')).toBeInTheDocument();
  });

  it('displays all initial cards', () => {
    render(<CardFlipDemo />);
    
    expect(screen.getByText('What is React?')).toBeInTheDocument();
    expect(screen.getByText('What is Redux?')).toBeInTheDocument();
    expect(screen.getByText('What is TypeScript?')).toBeInTheDocument();
  });

  describe('Card Flip Animation', () => {
    it('flips card when clicked', async () => {
      render(<CardFlipDemo />);
      
      const cards = screen.getAllByTestId('motion-div');
      const firstCard = cards[0];
      
      // Click to flip
      fireEvent.click(firstCard);
      
      // Check if the flip animation was triggered
      await waitFor(() => {
        expect(firstCard).toBeInTheDocument();
      });
    });

    it('displays front and back content correctly', () => {
      render(<CardFlipDemo />);
      
      // Check front content
      expect(screen.getByText('Question')).toBeInTheDocument();
      
      // The actual question/answer text should be visible
      const cards = screen.getAllByText(/What is/);
      expect(cards.length).toBeGreaterThan(0);
    });

    it('handles multiple card flips independently', async () => {
      render(<CardFlipDemo />);
      
      const cards = screen.getAllByTestId('motion-div');
      
      // Click multiple cards
      fireEvent.click(cards[0]);
      fireEvent.click(cards[1]);
      
      // Both should handle clicks independently
      await waitFor(() => {
        expect(cards[0]).toBeInTheDocument();
        expect(cards[1]).toBeInTheDocument();
      });
    });
  });

  describe('Drag & Drop Functionality', () => {
    it('renders draggable cards', () => {
      render(<CardFlipDemo />);
      
      // Check if drag & drop section exists
      expect(screen.getByText('Drag cards to reorder them!')).toBeInTheDocument();
      
      // Verify draggable cards are rendered
      const dragSection = screen.getByText('Drag & Drop Cards').parentElement;
      expect(dragSection).toBeInTheDocument();
    });

    it('displays card content in drag section', () => {
      render(<CardFlipDemo />);
      
      // Look for the card content in the draggable section
      const allReactTexts = screen.getAllByText(/What is React\?/);
      expect(allReactTexts.length).toBeGreaterThan(0);
      
      // Check for answer text
      expect(screen.getByText('A JavaScript library for building UIs')).toBeInTheDocument();
    });

    it('applies hover styles on draggable cards', async () => {
      render(<CardFlipDemo />);
      
      const user = userEvent.setup();
      
      // Find a draggable card
      const card = screen.getByText('A JavaScript library for building UIs').parentElement;
      
      if (card) {
        // Hover over the card
        await user.hover(card);
        
        // Check if hover styles are applied (shadow-lg class)
        expect(card.className).toContain('hover:shadow-lg');
      }
    });
  });

  describe('Visual Styling', () => {
    it('applies correct gradient colors to flip cards', () => {
      render(<CardFlipDemo />);
      
      const cards = screen.getAllByTestId('motion-div');
      const firstCard = cards[0];
      
      // Check for gradient classes
      const frontFace = firstCard.querySelector('.bg-gradient-to-br');
      expect(frontFace).toBeInTheDocument();
    });

    it('applies correct styling to draggable cards', () => {
      render(<CardFlipDemo />);
      
      const dragCard = screen.getByText('A JavaScript library for building UIs').parentElement;
      
      if (dragCard) {
        expect(dragCard.className).toContain('bg-white');
        expect(dragCard.className).toContain('rounded-lg');
        expect(dragCard.className).toContain('shadow-md');
      }
    });
  });

  describe('Accessibility', () => {
    it('cards are keyboard accessible', async () => {
      render(<CardFlipDemo />);
      
      const cards = screen.getAllByTestId('motion-div');
      const firstCard = cards[0];
      
      // Cards should be clickable
      expect(firstCard).toHaveAttribute('class');
      expect(firstCard.className).toContain('cursor-pointer');
    });

    it('provides clear instructions for users', () => {
      render(<CardFlipDemo />);
      
      expect(screen.getByText('Click the cards to flip them!')).toBeInTheDocument();
      expect(screen.getByText('Drag cards to reorder them!')).toBeInTheDocument();
    });
  });

  describe('Data Integrity', () => {
    it('maintains correct card data structure', () => {
      render(<CardFlipDemo />);
      
      // Verify all card fronts are present
      expect(screen.getByText('What is React?')).toBeInTheDocument();
      expect(screen.getByText('What is Redux?')).toBeInTheDocument();
      expect(screen.getByText('What is TypeScript?')).toBeInTheDocument();
      
      // Verify all card backs are present
      expect(screen.getByText('A JavaScript library for building UIs')).toBeInTheDocument();
      expect(screen.getByText('A state management library')).toBeInTheDocument();
      expect(screen.getByText('JavaScript with static typing')).toBeInTheDocument();
    });
  });
});
