import { ReactNode } from 'react';

// Base skeleton component with shimmer animation
export function Skeleton({ 
  className = '', 
  children 
}: { 
  className?: string; 
  children?: ReactNode;
}) {
  return (
    <div className={`animate-pulse ${className}`}>
      {children || <div className="bg-gray-200 rounded h-full w-full" />}
    </div>
  );
}

// Skeleton for a single card in a list
export function CardSkeleton() {
  return (
    <div className="card p-4">
      <Skeleton className="h-5 bg-gray-200 rounded w-3/4 mb-2" />
      <Skeleton className="h-4 bg-gray-200 rounded w-1/2" />
    </div>
  );
}

// Skeleton for deck card
export function DeckCardSkeleton() {
  return (
    <div className="card">
      <div className="flex justify-between items-start mb-3">
        <Skeleton className="h-6 bg-gray-200 rounded w-2/3" />
        <Skeleton className="h-6 bg-gray-200 rounded-full w-12" />
      </div>
      <Skeleton className="h-4 bg-gray-200 rounded w-full mb-2" />
      <Skeleton className="h-4 bg-gray-200 rounded w-3/4 mb-4" />
      <div className="flex justify-between items-center">
        <Skeleton className="h-4 bg-gray-200 rounded w-20" />
        <Skeleton className="h-8 bg-gray-200 rounded w-24" />
      </div>
    </div>
  );
}

// Skeleton for deck list
export function DeckListSkeleton({ count = 6 }: { count?: number }) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      {Array.from({ length: count }).map((_, i) => (
        <DeckCardSkeleton key={i} />
      ))}
    </div>
  );
}

// Skeleton for card list (in deck view)
export function CardListSkeleton({ count = 10 }: { count?: number }) {
  return (
    <div className="space-y-2">
      {Array.from({ length: count }).map((_, i) => (
        <CardSkeleton key={i} />
      ))}
    </div>
  );
}

// Skeleton for study session flashcard
export function FlashcardSkeleton() {
  return (
    <div className="study-session">
      {/* Progress Bar */}
      <div className="mb-6">
        <div className="flex justify-between text-sm mb-2">
          <Skeleton className="h-4 bg-gray-200 rounded w-24" />
          <Skeleton className="h-4 bg-gray-200 rounded w-20" />
        </div>
        <Skeleton className="h-2 bg-gray-200 rounded-full" />
      </div>

      {/* Flashcard */}
      <div className="h-96 mb-8">
        <Skeleton className="h-full bg-gray-200 rounded-lg" />
      </div>

      {/* Action buttons */}
      <div className="grid grid-cols-4 gap-3">
        {[...Array(4)].map((_, i) => (
          <Skeleton key={i} className="h-14 bg-gray-200 rounded-lg" />
        ))}
      </div>
    </div>
  );
}

// Skeleton for stats cards
export function StatCardSkeleton() {
  return (
    <div className="card">
      <div className="flex items-center gap-4">
        <Skeleton className="h-12 w-12 bg-gray-200 rounded-full flex-shrink-0" />
        <div className="flex-1">
          <Skeleton className="h-4 bg-gray-200 rounded w-24 mb-2" />
          <Skeleton className="h-8 bg-gray-200 rounded w-16" />
        </div>
      </div>
    </div>
  );
}

// Skeleton for a data table
export function TableSkeleton({ rows = 5, cols = 4 }: { rows?: number; cols?: number }) {
  return (
    <div className="overflow-x-auto">
      <table className="w-full">
        <thead>
          <tr>
            {Array.from({ length: cols }).map((_, i) => (
              <th key={i} className="p-3 text-left">
                <Skeleton className="h-4 bg-gray-200 rounded w-20" />
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {Array.from({ length: rows }).map((_, rowIndex) => (
            <tr key={rowIndex} className="border-t">
              {Array.from({ length: cols }).map((_, colIndex) => (
                <td key={colIndex} className="p-3">
                  <Skeleton className="h-4 bg-gray-200 rounded w-full" />
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
