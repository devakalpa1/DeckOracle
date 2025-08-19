import { Link, Outlet, useLocation, useNavigate } from 'react-router-dom';
import { useState } from 'react';
import { useAuth } from '../contexts/AuthContext';

const MainLayout = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { user, isAuthenticated, logout } = useAuth();
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);

  const navItems = [
    { path: '/', label: 'Home', icon: '🏠' },
    { path: '/decks', label: 'Decks', icon: '📚' },
    { path: '/progress', label: 'Progress', icon: '📊', requiresAuth: true },
  ];

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/';
    }
    return location.pathname.startsWith(path);
  };

  return (
    <div className="min-h-screen bg-[rgb(246_246_246)] flex flex-col">
      {/* Navigation */}
      <nav className="bg-white shadow-md border-b border-[rgb(176_215_225)]/30">
        <div className="container mx-auto px-4">
          <div className="flex justify-between items-center h-16">
            {/* Logo */}
            <Link to="/" className="flex items-center space-x-2">
              <span className="text-2xl">🔮</span>
              <span className="text-xl font-bold text-[rgb(18_55_64)]">DeckOracle</span>
            </Link>

            {/* Desktop Navigation */}
            <div className="hidden md:flex items-center space-x-8">
              {navItems
                .filter(item => !item.requiresAuth || isAuthenticated)
                .map((item) => (
                  <Link
                    key={item.path}
                    to={item.path}
                    className={`flex items-center space-x-1 px-3 py-2 rounded-lg transition-colors ${
                      isActive(item.path)
                        ? 'text-[rgb(84_154_171)] bg-[rgb(176_215_225)]/20'
                        : 'text-gray-600 hover:text-[rgb(84_154_171)] hover:bg-gray-50'
                    }`}
                  >
                    <span>{item.icon}</span>
                    <span>{item.label}</span>
                  </Link>
                ))}
              
              {/* Auth buttons */}
              {isAuthenticated ? (
                <div className="flex items-center space-x-4">
                  <span className="text-sm text-gray-600">
                    {user?.display_name || user?.email}
                  </span>
                  <button
                    onClick={logout}
                    className="text-sm text-gray-600 hover:text-red-600 transition-colors"
                  >
                    Logout
                  </button>
                </div>
              ) : (
                <div className="flex items-center space-x-4">
                  <Link
                    to="/login"
                    className="text-sm text-gray-600 hover:text-[rgb(84_154_171)] transition-colors"
                  >
                    Login
                  </Link>
                  <Link
                    to="/register"
                    className="btn-accent text-sm"
                  >
                    Sign Up
                  </Link>
                </div>
              )}
            </div>

            {/* Mobile menu button */}
            <button
              onClick={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
              className="md:hidden p-2 rounded-lg hover:bg-gray-100"
            >
              <svg
                className="w-6 h-6"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                {isMobileMenuOpen ? (
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M6 18L18 6M6 6l12 12"
                  />
                ) : (
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M4 6h16M4 12h16M4 18h16"
                  />
                )}
              </svg>
            </button>
          </div>

          {/* Mobile Navigation */}
          {isMobileMenuOpen && (
            <div className="md:hidden pb-4">
              <div className="flex flex-col space-y-2">
                {navItems
                  .filter(item => !item.requiresAuth || isAuthenticated)
                  .map((item) => (
                    <Link
                      key={item.path}
                      to={item.path}
                      onClick={() => setIsMobileMenuOpen(false)}
                      className={`flex items-center space-x-2 px-3 py-2 rounded-lg transition-colors ${
                        isActive(item.path)
                          ? 'text-[rgb(84_154_171)] bg-[rgb(176_215_225)]/20'
                          : 'text-gray-600 hover:text-[rgb(84_154_171)] hover:bg-gray-50'
                      }`}
                    >
                      <span>{item.icon}</span>
                      <span>{item.label}</span>
                    </Link>
                  ))}
                {isAuthenticated ? (
                  <div className="border-t pt-4 mt-4 space-y-2">
                    <div className="px-3 py-2 text-sm text-gray-600">
                      {user?.display_name || user?.email}
                    </div>
                    <button
                      onClick={() => {
                        logout();
                        setIsMobileMenuOpen(false);
                      }}
                      className="w-full text-left px-3 py-2 text-sm text-gray-600 hover:text-red-600 transition-colors"
                    >
                      Logout
                    </button>
                  </div>
                ) : (
                  <div className="border-t pt-4 mt-4 space-y-2">
                    <Link
                      to="/login"
                      onClick={() => setIsMobileMenuOpen(false)}
                      className="block px-3 py-2 text-sm text-center rounded-lg border border-gray-300 hover:bg-gray-50"
                    >
                      Login
                    </Link>
                    <Link
                      to="/register"
                      onClick={() => setIsMobileMenuOpen(false)}
                      className="btn-accent text-sm w-full text-center"
                    >
                      Sign Up
                    </Link>
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-1">
        <Outlet />
      </main>

      {/* Footer */}
      <footer className="bg-[rgb(18_55_64)] border-t border-[rgb(84_154_171)]/20 mt-auto">
        <div className="container mx-auto px-4 py-6">
          <div className="text-center text-[rgb(176_215_225)] text-sm">
            © 2024 DeckOracle. Master your learning journey.
          </div>
        </div>
      </footer>
    </div>
  );
};

export default MainLayout;
