import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react';
import type { Folder, Deck, Card } from '../../types';

export const api = createApi({
  reducerPath: 'deckOracleApi',
  baseQuery: fetchBaseQuery({ 
    baseUrl: 'http://localhost:8080/api/v1',
    prepareHeaders: (headers) => {
      // Get the token from localStorage
      const token = localStorage.getItem('token');
      if (token) {
        headers.set('authorization', `Bearer ${token}`);
      }
      return headers;
    },
  }),
  tagTypes: ['Folder', 'Deck', 'Card', 'StudySession', 'UserStats', 'Achievement', 'Progress', 'DeckProgress', 'CardPerformance', 'LearningCurve', 'StudyStreak', 'WeeklyProgress'],
  endpoints: (builder) => ({
    // Folders
    getFolders: builder.query<Folder[], void>({
      query: () => '/folders',
      providesTags: ['Folder'],
    }),
    getFolder: builder.query<Folder, string>({
      query: (id) => `/folders/${id}`,
      providesTags: (_result, _error, id) => [{ type: 'Folder', id }],
    }),
    createFolder: builder.mutation<Folder, Partial<Folder>>({
      query: (folder) => ({
        url: '/folders',
        method: 'POST',
        body: folder,
      }),
      invalidatesTags: ['Folder'],
    }),
    updateFolder: builder.mutation<Folder, { id: string; updates: Partial<Folder> }>({
      query: ({ id, updates }) => ({
        url: `/folders/${id}`,
        method: 'PUT',
        body: updates,
      }),
      invalidatesTags: (_result, _error, { id }) => [{ type: 'Folder', id }, 'Folder'],
    }),
    deleteFolder: builder.mutation<void, string>({
      query: (id) => ({
        url: `/folders/${id}`,
        method: 'DELETE',
      }),
      invalidatesTags: ['Folder'],
    }),

    // Decks
    getDecks: builder.query<Deck[], void>({
      query: () => '/decks',
      providesTags: ['Deck'],
    }),
    getDeck: builder.query<Deck, string>({
      query: (id) => `/decks/${id}`,
      providesTags: (_result, _error, id) => [{ type: 'Deck', id }],
    }),
    getDecksByFolder: builder.query<Deck[], string>({
      query: (folderId) => `/folders/${folderId}/decks`,
      providesTags: ['Deck'],
    }),
    createDeck: builder.mutation<Deck, Partial<Deck>>({
      query: (deck) => ({
        url: '/decks',
        method: 'POST',
        body: deck,
      }),
      invalidatesTags: ['Deck', 'Folder'],
    }),
    updateDeck: builder.mutation<Deck, { id: string; updates: Partial<Deck> }>({
      query: ({ id, updates }) => ({
        url: `/decks/${id}`,
        method: 'PUT',
        body: updates,
      }),
      invalidatesTags: (_result, _error, { id }) => [{ type: 'Deck', id }, 'Deck'],
    }),
    deleteDeck: builder.mutation<void, string>({
      query: (id) => ({
        url: `/decks/${id}`,
        method: 'DELETE',
      }),
      invalidatesTags: ['Deck', 'Folder'],
    }),

    // Cards
    getCards: builder.query<Card[], string>({
      query: (deckId) => `/cards?deck_id=${deckId}`,
      providesTags: ['Card'],
    }),
    getCard: builder.query<Card, string>({
      query: (id) => `/cards/${id}`,
      providesTags: (_result, _error, id) => [{ type: 'Card', id }],
    }),
    createCard: builder.mutation<Card, { deck_id: string; front: string; back: string; position?: number }>({
      query: ({ deck_id, ...cardData }) => ({
        url: `/cards?deck_id=${deck_id}`,
        method: 'POST',
        body: cardData,
      }),
      invalidatesTags: ['Card', 'Deck'],
    }),
    updateCard: builder.mutation<Card, { id: string; updates: Partial<Card> }>({
      query: ({ id, updates }) => ({
        url: `/cards/${id}`,
        method: 'PUT',
        body: updates,
      }),
      invalidatesTags: (_result, _error, { id }) => [{ type: 'Card', id }, 'Card'],
    }),
    deleteCard: builder.mutation<void, string>({
      query: (id) => ({
        url: `/cards/${id}`,
        method: 'DELETE',
      }),
      invalidatesTags: ['Card', 'Deck'],
    }),

    // Study Session endpoints
    createStudySession: builder.mutation<any, { deck_id: string; study_mode?: string }>({
      query: (data) => ({
        url: '/study/sessions',
        method: 'POST',
        body: data,
      }),
      invalidatesTags: ['StudySession'],
    }),
    getStudySession: builder.query<any, string>({
      query: (id) => `/study/sessions/${id}`,
      providesTags: (_result, _error, id) => [{ type: 'StudySession', id }],
    }),
    getStudySessions: builder.query<any[], void>({
      query: () => '/study/sessions',
      providesTags: (result) =>
        result
          ? [...result.map(({ id }) => ({ type: 'StudySession' as const, id })), 'StudySession']
          : ['StudySession'],
    }),
    completeStudySession: builder.mutation<any, string>({
      query: (id) => ({
        url: `/study/sessions/${id}/complete`,
        method: 'POST',
      }),
      invalidatesTags: (_result, _error, id) => [{ type: 'StudySession', id }, 'UserStats'],
    }),
    submitCardAnswer: builder.mutation<any, {
      sessionId: string;
      card_id: string;
      status: 'easy' | 'medium' | 'hard' | 'forgot';
      response_time_ms?: number;
      user_answer?: string;
      is_correct?: boolean;
    }>({
      query: ({ sessionId, ...data }) => ({
        url: `/study/sessions/${sessionId}/progress`,
        method: 'POST',
        body: data,
      }),
      invalidatesTags: (_result, _error, { sessionId }) => [{ type: 'StudySession', id: sessionId }],
    }),
    getSessionProgress: builder.query<any[], string>({
      query: (sessionId) => `/study/sessions/${sessionId}/progress`,
      providesTags: (_result, _error, id) => [{ type: 'StudySession', id }],
    }),
    getUserStats: builder.query<any, void>({
      query: () => '/study/stats',
      providesTags: ['UserStats'],
    }),
    getUserAchievements: builder.query<any[], void>({
      query: () => '/study/achievements',
      providesTags: ['Achievement'],
    }),

    // Progress Tracking endpoints
    getProgressOverview: builder.query<any, { deck_id?: string; start_date?: string; end_date?: string }>({
      query: (params) => ({
        url: '/progress/overview',
        params,
      }),
      providesTags: ['Progress'],
    }),
    getDeckProgress: builder.query<any[], void>({
      query: () => '/progress/decks',
      providesTags: (result) =>
        result
          ? [...result.map(({ deck_id }) => ({ type: 'DeckProgress' as const, id: deck_id })), 'DeckProgress']
          : ['DeckProgress'],
    }),
    getSpecificDeckProgress: builder.query<any, string>({
      query: (deckId) => `/progress/decks/${deckId}`,
      providesTags: (_result, _error, id) => [{ type: 'DeckProgress', id }],
    }),
    getCardPerformance: builder.query<any[], { deck_id?: string; start_date?: string; end_date?: string }>({
      query: (params) => ({
        url: '/progress/cards/performance',
        params,
      }),
      providesTags: ['CardPerformance'],
    }),
    getLearningCurve: builder.query<any[], { deck_id?: string; start_date?: string; end_date?: string }>({
      query: (params) => ({
        url: '/progress/learning-curve',
        params,
      }),
      providesTags: ['LearningCurve'],
    }),
    getStudyStreaks: builder.query<any, void>({
      query: () => '/progress/streaks',
      providesTags: ['StudyStreak'],
    }),
    getWeeklyProgress: builder.query<any[], void>({
      query: () => '/progress/weekly',
      providesTags: ['WeeklyProgress'],
    }),
  }),
});

export const {
  // Folders
  useGetFoldersQuery,
  useGetFolderQuery,
  useCreateFolderMutation,
  useUpdateFolderMutation,
  useDeleteFolderMutation,
  
  // Decks
  useGetDecksQuery,
  useGetDeckQuery,
  useGetDecksByFolderQuery,
  useCreateDeckMutation,
  useUpdateDeckMutation,
  useDeleteDeckMutation,
  
  // Cards
  useGetCardsQuery,
  useGetCardQuery,
  useCreateCardMutation,
  useUpdateCardMutation,
  useDeleteCardMutation,
  
  // Study Sessions
  useCreateStudySessionMutation,
  useGetStudySessionQuery,
  useGetStudySessionsQuery,
  useCompleteStudySessionMutation,
  useSubmitCardAnswerMutation,
  useGetSessionProgressQuery,
  useGetUserStatsQuery,
  useGetUserAchievementsQuery,
  
  // Progress Tracking
  useGetProgressOverviewQuery,
  useGetDeckProgressQuery,
  useGetSpecificDeckProgressQuery,
  useGetCardPerformanceQuery,
  useGetLearningCurveQuery,
  useGetStudyStreaksQuery,
  useGetWeeklyProgressQuery,
} = api;
