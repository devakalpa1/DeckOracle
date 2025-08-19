import React, { useState } from 'react';
import {
  LineChart,
  Line,
  BarChart,
  Bar,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from 'recharts';
import {
  useGetProgressOverviewQuery,
  useGetDeckProgressQuery,
  useGetLearningCurveQuery,
  useGetStudyStreaksQuery,
  useGetWeeklyProgressQuery,
  useGetCardPerformanceQuery,
} from '../store/services/api';
import './ProgressDashboard.css';

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884D8'];

const ProgressDashboard: React.FC = () => {
  const [selectedDeckId, setSelectedDeckId] = useState<string | undefined>(undefined);
  const [dateRange, setDateRange] = useState<{ start?: string; end?: string }>({});

  // Fetch data
  const { data: overview, isLoading: overviewLoading } = useGetProgressOverviewQuery({
    deck_id: selectedDeckId,
    ...dateRange,
  });
  const { data: deckProgress, isLoading: deckProgressLoading } = useGetDeckProgressQuery();
  const { data: learningCurve, isLoading: curveLoading } = useGetLearningCurveQuery({
    deck_id: selectedDeckId,
    ...dateRange,
  });
  const { data: streaks, isLoading: streaksLoading } = useGetStudyStreaksQuery();
  const { data: weeklyProgress, isLoading: weeklyLoading } = useGetWeeklyProgressQuery();
  const { data: cardPerformance, isLoading: cardPerfLoading } = useGetCardPerformanceQuery({
    deck_id: selectedDeckId,
    ...dateRange,
  });

  if (overviewLoading || deckProgressLoading || curveLoading || streaksLoading || weeklyLoading) {
    return (
      <div className="progress-dashboard loading">
        <div className="spinner">Loading progress data...</div>
      </div>
    );
  }

  // Format data for pie chart
  const deckMasteryData = deckProgress?.map((deck) => ({
    name: deck.deck_name,
    value: deck.mastery_percentage,
  })) || [];

  // Format learning curve data
  const formattedCurve = learningCurve?.map((item) => ({
    date: new Date(item.date).toLocaleDateString(),
    accuracy: item.accuracy,
    cards: item.cards_studied,
    time: item.study_time_minutes,
  })) || [];

  // Format weekly progress
  const formattedWeekly = weeklyProgress?.map((week) => ({
    week: new Date(week.week_start).toLocaleDateString(),
    cards: week.total_cards_studied,
    accuracy: week.average_accuracy,
    time: week.total_study_time_minutes,
    sessions: week.sessions_completed,
  })) || [];

  return (
    <div className="progress-dashboard">
      <div className="dashboard-header">
        <h1>Progress Dashboard</h1>
        <div className="dashboard-controls">
          <select
            value={selectedDeckId || ''}
            onChange={(e) => setSelectedDeckId(e.target.value || undefined)}
            className="deck-filter"
          >
            <option value="">All Decks</option>
            {deckProgress?.map((deck) => (
              <option key={deck.deck_id} value={deck.deck_id}>
                {deck.deck_name}
              </option>
            ))}
          </select>
          <div className="date-filters">
            <input
              type="date"
              placeholder="Start Date"
              onChange={(e) => setDateRange({ ...dateRange, start: e.target.value })}
            />
            <input
              type="date"
              placeholder="End Date"
              onChange={(e) => setDateRange({ ...dateRange, end: e.target.value })}
            />
          </div>
        </div>
      </div>

      {/* Overview Stats Cards */}
      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-icon">📚</div>
          <div className="stat-content">
            <h3>Total Cards Studied</h3>
            <p className="stat-value">{overview?.total_cards_studied || 0}</p>
          </div>
        </div>
        <div className="stat-card">
          <div className="stat-icon">⏱️</div>
          <div className="stat-content">
            <h3>Study Time</h3>
            <p className="stat-value">{overview?.total_study_time_minutes || 0} min</p>
          </div>
        </div>
        <div className="stat-card">
          <div className="stat-icon">🎯</div>
          <div className="stat-content">
            <h3>Average Accuracy</h3>
            <p className="stat-value">{(overview?.average_accuracy || 0).toFixed(1)}%</p>
          </div>
        </div>
        <div className="stat-card">
          <div className="stat-icon">🔥</div>
          <div className="stat-content">
            <h3>Current Streak</h3>
            <p className="stat-value">{streaks?.current_streak || 0} days</p>
          </div>
        </div>
        <div className="stat-card">
          <div className="stat-icon">📈</div>
          <div className="stat-content">
            <h3>Total Sessions</h3>
            <p className="stat-value">{overview?.total_sessions || 0}</p>
          </div>
        </div>
        <div className="stat-card">
          <div className="stat-icon">📚</div>
          <div className="stat-content">
            <h3>Decks in Progress</h3>
            <p className="stat-value">{overview?.decks_in_progress || 0}</p>
          </div>
        </div>
      </div>

      {/* Charts Section */}
      <div className="charts-grid">
        {/* Learning Curve Chart */}
        <div className="chart-card">
          <h3>Learning Curve (Last 30 Days)</h3>
          <ResponsiveContainer width="100%" height={300}>
            <LineChart data={formattedCurve}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="date" />
              <YAxis yAxisId="left" />
              <YAxis yAxisId="right" orientation="right" />
              <Tooltip />
              <Legend />
              <Line
                yAxisId="left"
                type="monotone"
                dataKey="accuracy"
                stroke="#8884d8"
                name="Accuracy %"
              />
              <Line
                yAxisId="right"
                type="monotone"
                dataKey="cards"
                stroke="#82ca9d"
                name="Cards Studied"
              />
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Weekly Progress Bar Chart */}
        <div className="chart-card">
          <h3>Weekly Progress</h3>
          <ResponsiveContainer width="100%" height={300}>
            <BarChart data={formattedWeekly}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="week" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Bar dataKey="cards" fill="#8884d8" name="Cards Studied" />
              <Bar dataKey="sessions" fill="#82ca9d" name="Sessions" />
            </BarChart>
          </ResponsiveContainer>
        </div>

        {/* Deck Mastery Pie Chart */}
        {deckMasteryData.length > 0 && (
          <div className="chart-card">
            <h3>Deck Mastery</h3>
            <ResponsiveContainer width="100%" height={300}>
              <PieChart>
                <Pie
                  data={deckMasteryData}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={(entry) => `${entry.name}: ${entry.value.toFixed(1)}%`}
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="value"
                >
                  {deckMasteryData.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip />
              </PieChart>
            </ResponsiveContainer>
          </div>
        )}

        {/* Study Streak Calendar */}
        <div className="chart-card">
          <h3>Study Streak</h3>
          <div className="streak-info">
            <div className="streak-stat">
              <span className="streak-label">Current Streak:</span>
              <span className="streak-value">{streaks?.current_streak || 0} days</span>
            </div>
            <div className="streak-stat">
              <span className="streak-label">Longest Streak:</span>
              <span className="streak-value">{streaks?.longest_streak || 0} days</span>
            </div>
          </div>
          <div className="streak-calendar">
            {streaks?.study_days?.map((day, index) => (
              <div
                key={index}
                className="streak-day"
                title={new Date(day).toLocaleDateString()}
              >
                ✓
              </div>
            ))}
          </div>
        </div>
      </div>

      {/* Deck Progress Table */}
      <div className="decks-progress-section">
        <h2>Deck Progress</h2>
        <div className="decks-table">
          <table>
            <thead>
              <tr>
                <th>Deck Name</th>
                <th>Total Cards</th>
                <th>Learned</th>
                <th>Reviewing</th>
                <th>New</th>
                <th>Mastery</th>
                <th>Accuracy</th>
                <th>Last Studied</th>
              </tr>
            </thead>
            <tbody>
              {deckProgress?.map((deck) => (
                <tr key={deck.deck_id}>
                  <td>{deck.deck_name}</td>
                  <td>{deck.total_cards}</td>
                  <td className="cards-learned">{deck.cards_learned}</td>
                  <td className="cards-reviewing">{deck.cards_reviewing}</td>
                  <td className="cards-new">{deck.cards_new}</td>
                  <td>
                    <div className="progress-bar">
                      <div
                        className="progress-fill"
                        style={{ width: `${deck.mastery_percentage}%` }}
                      />
                      <span>{deck.mastery_percentage.toFixed(1)}%</span>
                    </div>
                  </td>
                  <td>{deck.average_accuracy.toFixed(1)}%</td>
                  <td>
                    {deck.last_studied
                      ? new Date(deck.last_studied).toLocaleDateString()
                      : 'Never'}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Difficult Cards Section */}
      {cardPerformance && cardPerformance.length > 0 && (
        <div className="difficult-cards-section">
          <h2>Cards Needing Review</h2>
          <div className="difficult-cards-grid">
            {cardPerformance.slice(0, 6).map((card) => (
              <div key={card.card_id} className="difficult-card">
                <h4>{card.front}</h4>
                <div className="card-stats">
                  <span>Reviews: {card.total_reviews}</span>
                  <span>Accuracy: {card.accuracy_rate.toFixed(1)}%</span>
                  <span>Difficulty: {(card.difficulty_score * 100).toFixed(0)}%</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default ProgressDashboard;
