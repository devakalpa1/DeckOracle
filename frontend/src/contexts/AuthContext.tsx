import React, { createContext, useContext, useState, useEffect, useCallback } from 'react';
import axios, { AxiosError } from 'axios';
import { store } from '../store';
import { api as rtkApi } from '../store/services/api';

interface User {
  id: string;
  email: string;
  display_name?: string;
  email_verified: boolean;
  created_at: string;
}

interface AuthTokens {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
}

interface AuthResponse extends AuthTokens {
  user: User;
}

interface AuthContextType {
  user: User | null;
  loading: boolean;
  error: string | null;
  login: (email: string, password: string, rememberMe?: boolean) => Promise<void>;
  register: (email: string, password: string, displayName?: string) => Promise<void>;
  logout: () => Promise<void>;
  refreshToken: () => Promise<void>;
  clearError: () => void;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

// Create axios instance with interceptors
const api = axios.create({
  baseURL: `${API_BASE_URL}/api/v1`,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Token management
const TOKEN_KEY = 'auth_tokens';
const USER_KEY = 'auth_user';
const REMEMBER_ME_KEY = 'remember_me';

const saveTokens = (tokens: AuthTokens, rememberMe: boolean = false) => {
  const storage = rememberMe ? localStorage : sessionStorage;
  storage.setItem(TOKEN_KEY, JSON.stringify(tokens));
  // Also save just the access token for RTK Query compatibility
  storage.setItem('token', tokens.access_token);
  localStorage.setItem('token', tokens.access_token); // Always save to localStorage for RTK Query
  if (rememberMe) {
    localStorage.setItem(REMEMBER_ME_KEY, 'true');
  }
};

const getTokens = (): AuthTokens | null => {
  const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';
  const storage = rememberMe ? localStorage : sessionStorage;
  const tokens = storage.getItem(TOKEN_KEY);
  return tokens ? JSON.parse(tokens) : null;
};

const clearTokens = () => {
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
  localStorage.removeItem(REMEMBER_ME_KEY);
  localStorage.removeItem('token'); // Remove RTK Query token
  sessionStorage.removeItem(TOKEN_KEY);
  sessionStorage.removeItem(USER_KEY);
  sessionStorage.removeItem('token'); // Remove RTK Query token from session too
};

const saveUser = (user: User) => {
  const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';
  const storage = rememberMe ? localStorage : sessionStorage;
  storage.setItem(USER_KEY, JSON.stringify(user));
};

const getUser = (): User | null => {
  const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';
  const storage = rememberMe ? localStorage : sessionStorage;
  const user = storage.getItem(USER_KEY);
  return user ? JSON.parse(user) : null;
};

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(getUser());
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Setup axios interceptors
  useEffect(() => {
    // Request interceptor to add token
    const requestInterceptor = api.interceptors.request.use(
      (config) => {
        const tokens = getTokens();
        if (tokens) {
          config.headers.Authorization = `Bearer ${tokens.access_token}`;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor to handle token refresh
    const responseInterceptor = api.interceptors.response.use(
      (response) => response,
      async (error: AxiosError) => {
        const originalRequest = error.config;
        
        if (error.response?.status === 401 && originalRequest && !originalRequest._retry) {
          originalRequest._retry = true;
          
          try {
            await refreshToken();
            const tokens = getTokens();
            if (tokens) {
              originalRequest.headers.Authorization = `Bearer ${tokens.access_token}`;
            }
            return api(originalRequest);
          } catch (refreshError) {
            clearTokens();
            setUser(null);
            window.location.href = '/login';
            return Promise.reject(refreshError);
          }
        }
        
        return Promise.reject(error);
      }
    );

    return () => {
      api.interceptors.request.eject(requestInterceptor);
      api.interceptors.response.eject(responseInterceptor);
    };
  }, []);

  const refreshToken = useCallback(async () => {
    const tokens = getTokens();
    if (!tokens?.refresh_token) {
      throw new Error('No refresh token available');
    }

    try {
      const response = await api.post<AuthResponse>('/auth/refresh', {
        refresh_token: tokens.refresh_token,
      });

      const { user: newUser, ...newTokens } = response.data;
      const rememberMe = localStorage.getItem(REMEMBER_ME_KEY) === 'true';
      saveTokens(newTokens, rememberMe);
      saveUser(newUser);
      setUser(newUser);
    } catch (error) {
      clearTokens();
      setUser(null);
      throw error;
    }
  }, []);

  const login = useCallback(async (email: string, password: string, rememberMe: boolean = false) => {
    setError(null);
    setLoading(true);

    try {
      const response = await api.post<AuthResponse>('/auth/login', {
        email,
        password,
        remember_me: rememberMe,
      });

      const { user: loggedInUser, ...tokens } = response.data;
      saveTokens(tokens, rememberMe);
      saveUser(loggedInUser);
      setUser(loggedInUser);
      // Reset RTK Query cache to refetch with auth
      store.dispatch(rtkApi.util.resetApiState());
    } catch (error) {
      if (axios.isAxiosError(error)) {
        setError(error.response?.data?.error || 'Login failed');
      } else {
        setError('An unexpected error occurred');
      }
      throw error;
    } finally {
      setLoading(false);
    }
  }, []);

  const register = useCallback(async (email: string, password: string, displayName?: string) => {
    setError(null);
    setLoading(true);

    try {
      const response = await api.post<AuthResponse>('/auth/register', {
        email,
        password,
        display_name: displayName,
      });

      const { user: newUser, ...tokens } = response.data;
      saveTokens(tokens, false);
      saveUser(newUser);
      setUser(newUser);
      // Reset RTK Query cache to refetch with auth
      store.dispatch(rtkApi.util.resetApiState());
    } catch (error) {
      if (axios.isAxiosError(error)) {
        setError(error.response?.data?.error || 'Registration failed');
      } else {
        setError('An unexpected error occurred');
      }
      throw error;
    } finally {
      setLoading(false);
    }
  }, []);

  const logout = useCallback(async () => {
    setLoading(true);
    try {
      const tokens = getTokens();
      if (tokens) {
        await api.post('/auth/logout');
      }
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      clearTokens();
      setUser(null);
      setLoading(false);
      // Reset RTK Query cache after logout
      store.dispatch(rtkApi.util.resetApiState());
    }
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  // Auto-refresh token before expiration
  useEffect(() => {
    const tokens = getTokens();
    if (!tokens) {
      setLoading(false);
      return;
    }

    // Calculate when to refresh (5 minutes before expiration)
    const refreshTime = (tokens.expires_in - 300) * 1000;
    
    const timeout = setTimeout(async () => {
      try {
        await refreshToken();
      } catch (error) {
        console.error('Auto-refresh failed:', error);
      }
    }, refreshTime);

    setLoading(false);

    return () => clearTimeout(timeout);
  }, [refreshToken]);

  const value: AuthContextType = {
    user,
    loading,
    error,
    login,
    register,
    logout,
    refreshToken,
    clearError,
    isAuthenticated: !!user,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};

// Export the api instance for use in other parts of the app
export { api };
