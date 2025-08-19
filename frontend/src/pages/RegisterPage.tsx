import React, { useState, useEffect } from 'react';
import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/AuthContext";

export default function RegisterPage() {
  const navigate = useNavigate();
  const { register, error, clearError, isAuthenticated, loading: authLoading } = useAuth();
  
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [displayName, setDisplayName] = useState('');
  const [loading, setLoading] = useState(false);
  const [validationErrors, setValidationErrors] = useState<string[]>([]);

  useEffect(() => {
    if (isAuthenticated) {
      navigate('/decks', { replace: true });
    }
  }, [isAuthenticated, navigate]);

  useEffect(() => {
    clearError();
  }, [clearError]);

  const validatePassword = (pwd: string): string[] => {
    const errors: string[] = [];
    
    if (pwd.length < 8) {
      errors.push('Password must be at least 8 characters long');
    }
    if (!/[A-Z]/.test(pwd)) {
      errors.push('Password must contain at least one uppercase letter');
    }
    if (!/[a-z]/.test(pwd)) {
      errors.push('Password must contain at least one lowercase letter');
    }
    if (!/[0-9]/.test(pwd)) {
      errors.push('Password must contain at least one number');
    }
    
    return errors;
  };

  const handlePasswordChange = (value: string) => {
    setPassword(value);
    setValidationErrors(validatePassword(value));
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validate form
    const passwordErrors = validatePassword(password);
    if (passwordErrors.length > 0) {
      setValidationErrors(passwordErrors);
      return;
    }
    
    if (password !== confirmPassword) {
      setValidationErrors(['Passwords do not match']);
      return;
    }
    
    setValidationErrors([]);
    setLoading(true);

    try {
      await register(email, password, displayName || undefined);
      navigate('/decks', { replace: true });
    } catch (err) {
      // Error is handled by the auth context
    } finally {
      setLoading(false);
    }
  };

  if (authLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-[rgb(84_154_171)]"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-[rgb(176_215_225)] via-[rgb(246_246_246)] to-[rgb(230_244_247)] flex items-center justify-center px-4 py-12">
      <div className="max-w-md w-full space-y-8">
        {/* Logo and Title */}
        <div className="text-center">
          <div className="flex justify-center mb-4">
            <span className="text-5xl">🔮</span>
          </div>
          <h2 className="text-3xl font-bold text-[rgb(18_55_64)]">Create your account</h2>
          <p className="mt-2 text-gray-600">Join DeckOracle to start your learning journey</p>
        </div>
        
        {/* Registration Form */}
        <div className="card bg-white p-8">
          <form className="space-y-6" onSubmit={handleSubmit}>
            {/* Error Messages */}
            {(error || validationErrors.length > 0) && (
              <div className="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg">
                {error && <p className="text-sm">{error}</p>}
                {validationErrors.map((err, idx) => (
                  <p key={idx} className="text-sm">{err}</p>
                ))}
              </div>
            )}

            {/* Display Name Field (Optional) */}
            <div>
              <label htmlFor="displayName" className="block text-sm font-medium text-gray-700 mb-2">
                Display Name (Optional)
              </label>
              <input
                id="displayName"
                name="displayName"
                type="text"
                autoComplete="name"
                value={displayName}
                onChange={(e) => setDisplayName(e.target.value)}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="How should we call you?"
              />
            </div>

            {/* Email Field */}
            <div>
              <label htmlFor="email" className="block text-sm font-medium text-gray-700 mb-2">
                Email address *
              </label>
              <input
                id="email"
                name="email"
                type="email"
                autoComplete="email"
                required
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="you@example.com"
              />
            </div>
            
            {/* Password Field */}
            <div>
              <label htmlFor="password" className="block text-sm font-medium text-gray-700 mb-2">
                Password *
              </label>
              <input
                id="password"
                name="password"
                type="password"
                autoComplete="new-password"
                required
                value={password}
                onChange={(e) => handlePasswordChange(e.target.value)}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="••••••••"
              />
            </div>

            {/* Confirm Password Field */}
            <div>
              <label htmlFor="confirmPassword" className="block text-sm font-medium text-gray-700 mb-2">
                Confirm Password *
              </label>
              <input
                id="confirmPassword"
                name="confirmPassword"
                type="password"
                autoComplete="new-password"
                required
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                className="appearance-none rounded-lg relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-2 focus:ring-[rgb(84_154_171)] focus:border-transparent"
                placeholder="••••••••"
              />
            </div>

            {/* Password Requirements Info */}
            <div className="bg-blue-50 border border-blue-200 px-4 py-3 rounded-lg">
              <p className="text-sm text-blue-800 font-medium mb-2">
                ℹ️ Password Requirements:
              </p>
              <ul className="list-disc list-inside text-sm text-blue-700 space-y-1">
                <li>At least 8 characters</li>
                <li>One uppercase letter</li>
                <li>One lowercase letter</li>
                <li>One number</li>
              </ul>
            </div>

            {/* Submit Button */}
            <div>
              <button
                type="submit"
                disabled={loading}
                className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-lg text-white bg-gradient-to-r from-[rgb(84_154_171)] to-[rgb(116_188_208)] hover:from-[rgb(116_188_208)] hover:to-[rgb(84_154_171)] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[rgb(84_154_171)] disabled:opacity-50 disabled:cursor-not-allowed transition-all"
              >
                {loading ? (
                  <span className="flex items-center">
                    <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Creating account...
                  </span>
                ) : (
                  'Create Account'
                )}
              </button>
            </div>

            {/* Login Link */}
            <div className="text-center text-sm">
              <span className="text-gray-600">Already have an account? </span>
              <Link to="/login" className="font-medium text-[rgb(84_154_171)] hover:text-[rgb(116_188_208)]">
                Sign in
              </Link>
            </div>

            {/* Terms and Privacy */}
            <div className="text-center text-xs text-gray-500">
              By creating an account, you agree to our{' '}
              <Link to="/terms" className="text-[rgb(84_154_171)] hover:underline">
                Terms of Service
              </Link>{' '}
              and{' '}
              <Link to="/privacy" className="text-[rgb(84_154_171)] hover:underline">
                Privacy Policy
              </Link>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
