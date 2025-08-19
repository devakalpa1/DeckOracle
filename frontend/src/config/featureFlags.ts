import React from 'react';

/**
 * Feature flags configuration for DeckOracle
 * Controls which features are enabled/disabled in the application
 */

export interface FeatureFlags {
  // Study Features
  spacedRepetition: boolean;
  adaptiveLearning: boolean;
  
  // Gamification
  achievements: boolean;
  leaderboards: boolean;
  dailyGoals: boolean;
  streaks: boolean;
  
  // Analytics
  detailedStats: boolean;
  learningCurve: boolean;
  progressTracking: boolean;
  
  // Social Features
  deckSharing: boolean;
  publicDecks: boolean;
  comments: boolean;
  
  // Advanced Features
  aiCardGeneration: boolean;
  smartSuggestions: boolean;
  voiceCards: boolean;
  imageOcr: boolean;
  
  // Import/Export
  ankiImport: boolean;
  quizletImport: boolean;
  csvExport: boolean;
  pdfExport: boolean;
}

// Default feature flags configuration
const defaultFlags: FeatureFlags = {
  // Core features - enabled
  spacedRepetition: false,  // Coming soon
  adaptiveLearning: false,  // Coming soon
  
  // Gamification - partial
  achievements: false,  // Backend stub exists but not functional
  leaderboards: false,  // Not implemented
  dailyGoals: false,    // Not implemented
  streaks: false,       // Backend stub exists
  
  // Analytics - partial
  detailedStats: false,  // Backend stubs exist
  learningCurve: false,  // Backend stub exists
  progressTracking: true, // Basic version works
  
  // Social - not implemented
  deckSharing: false,
  publicDecks: true,   // Basic support exists
  comments: false,
  
  // Advanced - not implemented
  aiCardGeneration: false,
  smartSuggestions: false,
  voiceCards: false,
  imageOcr: false,
  
  // Import/Export - partial
  ankiImport: false,
  quizletImport: false,
  csvExport: true,     // Basic support exists
  pdfExport: false,
};

// Override flags from environment variables (for development/testing)
const envFlags: Partial<FeatureFlags> = {};
if (process.env.NODE_ENV === 'development') {
  // In development, you can enable features via env vars
  // Example: REACT_APP_FEATURE_ACHIEVEMENTS=true
  Object.keys(defaultFlags).forEach(key => {
    const envKey = `REACT_APP_FEATURE_${key.toUpperCase()}`;
    const envValue = process.env[envKey];
    if (envValue !== undefined) {
      (envFlags as any)[key] = envValue === 'true';
    }
  });
}

// Merge defaults with environment overrides
export const featureFlags: FeatureFlags = {
  ...defaultFlags,
  ...envFlags,
};

/**
 * Hook to check if a feature is enabled
 */
export function useFeatureFlag(flag: keyof FeatureFlags): boolean {
  return featureFlags[flag];
}

/**
 * Check if any of the provided features are enabled
 */
export function hasAnyFeature(...flags: (keyof FeatureFlags)[]): boolean {
  return flags.some(flag => featureFlags[flag]);
}

/**
 * Check if all of the provided features are enabled
 */
export function hasAllFeatures(...flags: (keyof FeatureFlags)[]): boolean {
  return flags.every(flag => featureFlags[flag]);
}

/**
 * Component to conditionally render based on feature flag
 */
export function FeatureFlag({ 
  flag, 
  children, 
  fallback = null 
}: { 
  flag: keyof FeatureFlags;
  children: React.ReactNode;
  fallback?: React.ReactNode;
}) {
  return featureFlags[flag] ? React.createElement(React.Fragment, null, children) : React.createElement(React.Fragment, null, fallback);
}
