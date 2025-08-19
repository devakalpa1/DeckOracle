import axios, { AxiosInstance } from 'axios';

const API_BASE_URL = 'http://localhost:8080/api/v1';

// Create axios instance with base configuration
const axiosInstance: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add request interceptor to include auth token
axiosInstance.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Add response interceptor to handle 401 errors
axiosInstance.interceptors.response.use(
  (response) => response,
  async (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      localStorage.removeItem('user');
      
      // Redirect to login page if not already there
      if (window.location.pathname !== '/login' && window.location.pathname !== '/register') {
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

// Auth interfaces
export interface LoginCredentials {
  email: string;
  password: string;
  remember_me?: boolean;
}

export interface RegisterData {
  email: string;
  password: string;
  display_name?: string;
}

export interface User {
  id: string;
  email: string;
  display_name?: string;
  email_verified: boolean;
  created_at: string;
}

export interface AuthResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
  user: User;
}

// Auth service functions
export const authService = {
  async login(credentials: LoginCredentials): Promise<AuthResponse> {
    const response = await axiosInstance.post<AuthResponse>('/auth/login', credentials);
    const data = response.data;
    
    // Store tokens and user info
    localStorage.setItem('access_token', data.access_token);
    localStorage.setItem('refresh_token', data.refresh_token);
    localStorage.setItem('user', JSON.stringify(data.user));
    
    return data;
  },

  async register(data: RegisterData): Promise<AuthResponse> {
    const response = await axiosInstance.post<AuthResponse>('/auth/register', data);
    const responseData = response.data;
    
    // Store tokens and user info
    localStorage.setItem('access_token', responseData.access_token);
    localStorage.setItem('refresh_token', responseData.refresh_token);
    localStorage.setItem('user', JSON.stringify(responseData.user));
    
    return responseData;
  },

  async refreshToken(): Promise<AuthResponse> {
    const refreshToken = localStorage.getItem('refresh_token');
    if (!refreshToken) {
      throw new Error('No refresh token available');
    }
    
    const response = await axiosInstance.post<AuthResponse>('/auth/refresh', {
      refresh_token: refreshToken,
    });
    const data = response.data;
    
    // Update tokens
    localStorage.setItem('access_token', data.access_token);
    localStorage.setItem('refresh_token', data.refresh_token);
    
    return data;
  },

  logout(): void {
    // Clear all auth data
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
    
    // Redirect to home page
    window.location.href = '/';
  },

  getCurrentUser(): User | null {
    const userStr = localStorage.getItem('user');
    if (!userStr) return null;
    
    try {
      return JSON.parse(userStr);
    } catch {
      return null;
    }
  },

  getAccessToken(): string | null {
    return localStorage.getItem('access_token');
  },

  isAuthenticated(): boolean {
    return !!localStorage.getItem('access_token');
  },
};

// Export the axios instance for use in other services
export default axiosInstance;
