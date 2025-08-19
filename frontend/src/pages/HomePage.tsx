import { Link } from 'react-router-dom';
import CardFlipDemo from '../components/CardFlipDemo';

const HomePage = () => {
  return (
    <div className="container mx-auto px-4 py-8">
      <div className="max-w-4xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold text-[rgb(18_55_64)] mb-4">
            Welcome to DeckOracle
          </h1>
          <p className="text-xl text-gray-600 mb-8">
            Master any subject with smart flashcards and spaced repetition
          </p>
          <div className="flex gap-4 justify-center">
            <Link
              to="/decks"
              className="btn-primary inline-block"
            >
              Browse Decks
            </Link>
            <button className="btn-accent">
              Create New Deck
            </button>
          </div>
        </div>

        <div className="grid md:grid-cols-3 gap-6 mt-16">
          <div className="card text-center hover:border-[rgb(84_154_171)]/50 transition-all">
            <div className="text-4xl mb-4">📚</div>
            <h3 className="text-xl font-semibold mb-2 text-[rgb(18_55_64)]">Organize Your Study</h3>
            <p className="text-gray-600">
              Create folders and decks to organize your learning materials efficiently
            </p>
          </div>
          <div className="card text-center hover:border-[rgb(84_154_171)]/50 transition-all">
            <div className="text-4xl mb-4">🎯</div>
            <h3 className="text-xl font-semibold mb-2 text-[rgb(18_55_64)]">Smart Learning</h3>
            <p className="text-gray-600">
              Use spaced repetition algorithms to optimize your study sessions
            </p>
          </div>
          <div className="card text-center hover:border-[rgb(84_154_171)]/50 transition-all">
            <div className="text-4xl mb-4">📊</div>
            <h3 className="text-xl font-semibold mb-2 text-[rgb(18_55_64)]">Track Progress</h3>
            <p className="text-gray-600">
              Monitor your learning progress with detailed analytics and insights
            </p>
          </div>
        </div>

        <div className="mt-16 p-8 bg-[rgb(176_215_225)]/20 rounded-lg border border-[rgb(176_215_225)]/50">
          <h2 className="text-2xl font-bold text-[rgb(18_55_64)] mb-6">Quick Stats</h2>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div className="bg-white rounded-lg p-4 text-center shadow-sm">
              <div className="text-3xl font-bold text-[rgb(84_154_171)]">0</div>
              <div className="text-gray-600 text-sm mt-1">Total Decks</div>
            </div>
            <div className="bg-white rounded-lg p-4 text-center shadow-sm">
              <div className="text-3xl font-bold text-[rgb(84_154_171)]">0</div>
              <div className="text-gray-600 text-sm mt-1">Total Cards</div>
            </div>
            <div className="bg-white rounded-lg p-4 text-center shadow-sm">
              <div className="text-3xl font-bold text-[rgb(241_128_45)]">0</div>
              <div className="text-gray-600 text-sm mt-1">Cards Due</div>
            </div>
            <div className="bg-white rounded-lg p-4 text-center shadow-sm">
              <div className="text-3xl font-bold text-[rgb(241_128_45)]">0</div>
              <div className="text-gray-600 text-sm mt-1">Study Streak</div>
            </div>
          </div>
        </div>

        {/* Interactive Demo Section */}
        <div className="mt-16">
          <CardFlipDemo />
        </div>
      </div>
    </div>
  );
};

export default HomePage;
