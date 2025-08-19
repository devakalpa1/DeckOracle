// Type definitions for DeckOracle entities

export interface Folder {
  id: string;
  name: string;
  description?: string;
  color?: string;
  parentId?: string;
  deckCount?: number;
  createdAt: string;
  updatedAt: string;
}

export interface Deck {
  id: string;
  name: string;
  description?: string;
  folderId?: string;
  cardCount?: number;
  tags?: string[];
  isPublic: boolean;
  createdAt: string;
  updatedAt: string;
}

// Extended deck type with statistics
export interface DeckWithStats extends Deck {
  lastStudied?: string;
  cardsLearned?: number;
  cardsReviewing?: number;
  cardsNew?: number;
}

export interface Card {
  id: string;
  deckId: string;
  front: string;
  back: string;
  position: number;  // Added from backend model
  tags?: string[];  // TODO: backend-sync - not in backend yet
  createdAt: string;
  updatedAt: string;
}

export interface StudySession {
  id: string;
  deckId: string;
  startTime: string;
  endTime?: string;
  cardsStudied: number;
  correctAnswers: number;
  wrongAnswers: number;
}

export interface User {
  id: string;
  username: string;
  email: string;
  avatarUrl?: string;
  createdAt: string;
}

// API Response Types
export interface ApiResponse<T> {
  data: T;
  message?: string;
  success: boolean;
}

export interface PaginatedResponse<T> {
  items: T[];
  total: number;
  page: number;
  pageSize: number;
  hasNext: boolean;
  hasPrevious: boolean;
}
