import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { AuthProvider } from './contexts/AuthContext';
import MainLayout from './layouts/MainLayout';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import RegisterPage from './pages/RegisterPage';
import DeckListPage from './pages/DeckListPage';
import DeckViewPage from './pages/DeckViewPage';
import StudyPage from './pages/StudyPage';
import ProgressDashboard from './pages/ProgressDashboard';
import ProtectedRoute from './components/ProtectedRoute';

function App() {
  return (
    <AuthProvider>
      <BrowserRouter>
        <Routes>
          {/* Public routes */}
          <Route element={<MainLayout />}>
            <Route path="/" element={<HomePage />} />
            <Route path="/login" element={<LoginPage />} />
            <Route path="/register" element={<RegisterPage />} />
            
            {/* Protected routes */}
            <Route element={<ProtectedRoute />}>
              <Route path="/decks" element={<DeckListPage />} />
              <Route path="/decks/:id" element={<DeckViewPage />} />
              <Route path="/study/:deckId" element={<StudyPage />} />
              <Route path="/progress" element={<ProgressDashboard />} />
            </Route>
          </Route>
        </Routes>
      </BrowserRouter>
    </AuthProvider>
  );
}

export default App
