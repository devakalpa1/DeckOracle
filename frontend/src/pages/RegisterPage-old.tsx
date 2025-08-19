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
      navigate('/dashboard', { replace: true });
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
      navigate('/dashboard', { replace: true });
    } catch (err) {
      // Error is handled by the auth context
    } finally {
      setLoading(false);
    }
  };

  if (authLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <Loader2 className="h-8 w-8 animate-spin text-primary" />
      </div>
    );
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md w-full space-y-8">
        <div>
          <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
            Create your account
          </h2>
          <p className="mt-2 text-center text-sm text-gray-600">
            Or{' '}
            <Link
              to="/login"
              className="font-medium text-primary hover:text-primary-dark"
            >
              sign in to your existing account
            </Link>
          </p>
        </div>
        
        <form className="mt-8 space-y-6" onSubmit={handleSubmit}>
          {(error || validationErrors.length > 0) && (
            <div className="rounded-md bg-red-50 p-4">
              <div className="flex">
                <div className="flex-shrink-0">
                  <AlertCircle className="h-5 w-5 text-red-400" />
                </div>
                <div className="ml-3">
                  {error && <p className="text-sm font-medium text-red-800">{error}</p>}
                  {validationErrors.map((err, idx) => (
                    <p key={idx} className="text-sm font-medium text-red-800">{err}</p>
                  ))}
                </div>
              </div>
            </div>
          )}

          <div className="space-y-4">
            <div>
              <label htmlFor="displayName" className="sr-only">
                Display Name (Optional)
              </label>
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <User className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="displayName"
                  name="displayName"
                  type="text"
                  autoComplete="name"
                  value={displayName}
                  onChange={(e) => setDisplayName(e.target.value)}
                  className="appearance-none relative block w-full px-10 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                  placeholder="Display Name (Optional)"
                />
              </div>
            </div>

            <div>
              <label htmlFor="email" className="sr-only">
                Email address
              </label>
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <Mail className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="email"
                  name="email"
                  type="email"
                  autoComplete="email"
                  required
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  className="appearance-none relative block w-full px-10 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                  placeholder="Email address"
                />
              </div>
            </div>
            
            <div>
              <label htmlFor="password" className="sr-only">
                Password
              </label>
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <Lock className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="password"
                  name="password"
                  type="password"
                  autoComplete="new-password"
                  required
                  value={password}
                  onChange={(e) => handlePasswordChange(e.target.value)}
                  className="appearance-none relative block w-full px-10 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                  placeholder="Password"
                />
              </div>
            </div>

            <div>
              <label htmlFor="confirmPassword" className="sr-only">
                Confirm Password
              </label>
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <Lock className="h-5 w-5 text-gray-400" />
                </div>
                <input
                  id="confirmPassword"
                  name="confirmPassword"
                  type="password"
                  autoComplete="new-password"
                  required
                  value={confirmPassword}
                  onChange={(e) => setConfirmPassword(e.target.value)}
                  className="appearance-none relative block w-full px-10 py-3 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-md focus:outline-none focus:ring-primary focus:border-primary focus:z-10 sm:text-sm"
                  placeholder="Confirm Password"
                />
              </div>
            </div>
          </div>

          <div className="rounded-md bg-blue-50 p-4">
            <div className="flex">
              <div className="flex-shrink-0">
                <Info className="h-5 w-5 text-blue-400" />
              </div>
              <div className="ml-3">
                <p className="text-sm text-blue-700">
                  Password must contain:
                </p>
                <ul className="list-disc list-inside text-sm text-blue-700 mt-1">
                  <li>At least 8 characters</li>
                  <li>One uppercase letter</li>
                  <li>One lowercase letter</li>
                  <li>One number</li>
                </ul>
              </div>
            </div>
          </div>

          <div>
            <button
              type="submit"
              disabled={loading}
              className="group relative w-full flex justify-center py-3 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary hover:bg-primary-dark focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {loading ? (
                <Loader2 className="h-5 w-5 animate-spin" />
              ) : (
                'Create Account'
              )}
            </button>
          </div>

          <div className="text-center text-sm text-gray-600">
            By creating an account, you agree to our{' '}
            <Link to="/terms" className="font-medium text-primary hover:text-primary-dark">
              Terms of Service
            </Link>{' '}
            and{' '}
            <Link to="/privacy" className="font-medium text-primary hover:text-primary-dark">
              Privacy Policy
            </Link>
          </div>
        </form>
      </div>
    </div>
  );
}
