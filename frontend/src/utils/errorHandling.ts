/**
 * Standardized error handling utilities for the application
 */

interface ApiError {
  message?: string;
  error?: string;
  status?: number;
  details?: any;
}

/**
 * Extract error message from various error formats
 */
export function getErrorMessage(error: any): string {
  // Handle RTK Query errors
  if (error?.data) {
    // Check for backend error format
    if (typeof error.data === 'object') {
      return error.data.error || error.data.message || 'An error occurred';
    }
    if (typeof error.data === 'string') {
      return error.data;
    }
  }

  // Handle Axios errors
  if (error?.response?.data) {
    const data = error.response.data;
    return data.error || data.message || 'An error occurred';
  }

  // Handle standard Error objects
  if (error?.message) {
    return error.message;
  }

  // Fallback
  return 'An unexpected error occurred';
}

/**
 * Check if error is a network error
 */
export function isNetworkError(error: any): boolean {
  return error?.status === 'FETCH_ERROR' || 
         error?.message?.toLowerCase().includes('network') ||
         error?.message?.toLowerCase().includes('fetch');
}

/**
 * Check if error is an authentication error
 */
export function isAuthError(error: any): boolean {
  const status = error?.status || error?.response?.status;
  return status === 401 || status === 403;
}

/**
 * Check if error is a validation error
 */
export function isValidationError(error: any): boolean {
  const status = error?.status || error?.response?.status;
  return status === 400 || status === 422;
}

/**
 * Format error for display to user
 */
export function formatErrorForDisplay(error: any): {
  title: string;
  message: string;
  type: 'error' | 'warning' | 'info';
} {
  if (isNetworkError(error)) {
    return {
      title: 'Connection Error',
      message: 'Unable to connect to the server. Please check your internet connection.',
      type: 'warning'
    };
  }

  if (isAuthError(error)) {
    return {
      title: 'Authentication Required',
      message: 'Please log in to continue.',
      type: 'info'
    };
  }

  if (isValidationError(error)) {
    return {
      title: 'Validation Error',
      message: getErrorMessage(error),
      type: 'warning'
    };
  }

  return {
    title: 'Error',
    message: getErrorMessage(error),
    type: 'error'
  };
}

/**
 * Log error for debugging (in development only)
 */
export function logError(error: any, context?: string): void {
  if (process.env.NODE_ENV === 'development') {
    console.group(`🔴 Error${context ? ` in ${context}` : ''}`);
    console.error('Error object:', error);
    console.error('Message:', getErrorMessage(error));
    console.error('Stack:', error?.stack);
    console.groupEnd();
  }
}
