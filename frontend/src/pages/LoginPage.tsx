import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useAuth } from '../contexts/AuthContext';

const LoginPage = () => {
  const navigate = useNavigate();
  const { login, error, clearError } = useAuth();
  
  const [formData, setFormData] = useState({
    email: '',
    password: '',
    rememberMe: false,
  });
  const [isLoading, setIsLoading] = useState(false);
  const [localError, setLocalError] = useState('');

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }));
    setLocalError('');
    if (error) clearError();
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    setLocalError('');
    
    try {
      await login(formData.email, formData.password, formData.rememberMe);
      navigate('/decks');
    } catch (err: any) {
      console.error('Login error:', err);
      setLocalError(err.response?.data?.error || 'Login failed. Please check your credentials.');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-[rgb(176_215_225)] via-[rgb(246_246_246)] to-[rgb(230_244_247)] flex items-center justify-center px-4 py-12">
      <div className="max-w-md w-full space-y-8">
        {/* Logo and Title */}
        <div className="text-center">
          <div className="flex justify-center mb-4">
            <span className="text-5xl">🔮</span>
          </div>
          <h2 className="text-3xl font-bold text-[rgb(18_55_64)]">Welcome back!</h2>
          <p className="mt-2 text-gray-600">Sign in to continue your learning journey</p>
        </div>

        {/* Login Form */}
        <div className="card bg-white p-8">
          <form className="space-y-6" onSubmit={handleSubmit}>
            {/* Error Message */}
            {(error || localError) && (
              <div className="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg">
                {error || localError}
              </div>
            )}

            {/* Email Field */}
            <div>
              <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-2">
                Email address
              </label>
              <input
                id="email"
                name="email"
                type="email"
                autoComplete="email"
                required
                value={formData.email}
                onChange={handleChange}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="you@example.com"
              />
            </div>

            {/* Password Field */}
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-2">
                Password
              </label>
              <input
                id="password"
                name="password"
                type="password"
                autoComplete="current-password"
                required
                value={formData.password}
                onChange={handleChange}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="••••••••"
              />
            </div>

            {/* Remember Me Checkbox */}
            <div className="flex items-center justify-between">
              <div className="flex items-center">
                <input
                  id="rememberMe"
                  name="rememberMe"
                  type="checkbox"
                  checked={formData.rememberMe}
                  onChange={handleChange}
                  className="h-4 w-4 text-[rgb(84_154_171)] focus:ring-[rgb(84_154_171)] border-gray-300 rounded"
                />
                <label htmlFor="rememberMe" className="ml-2 block text-sm text-gray-700">
                  Remember me
                </label>
              </div>
              <Link to="/forgot-password" className="text-sm text-[rgb(84_154_171)] hover:text-[rgb(116_188_208)]">
                Forgot password?
              </Link>
            </div>

            {/* Submit Button */}
            <div>
              <button
                type="submit"
                disabled={isLoading}
                className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-gradient-to-r from-[rgb(84_154_171)] to-[rgb(116_188_208)] hover:from-[rgb(116_188_208)] hover:to-[rgb(84_154_171)] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[rgb(84_154_171)] disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              >
                {isLoading ? (
                  <span className="flex items-center">
                    <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Signing in...
                  </span>
                ) : (
                  'Sign in'
                )}
              </button>
            </div>

            {/* Register Link */}
            <div className="text-center text-sm">
              <span className="text-gray-600">Don't have an account? </span>
              <Link to="/register" className="font-medium text-[rgb(84_154_171)] hover:text-[rgb(116_188_208)]">
                Sign up now
              </Link>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
};

export default LoginPage;
