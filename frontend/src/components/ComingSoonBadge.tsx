import { ReactNode } from 'react';

interface ComingSoonBadgeProps {
  inline?: boolean;
  className?: string;
}

export function ComingSoonBadge({ inline = false, className = '' }: ComingSoonBadgeProps) {
  const baseClasses = inline 
    ? 'inline-flex items-center px-2 py-0.5 text-xs' 
    : 'inline-flex items-center px-3 py-1 text-sm';
    
  return (
    <span className={`${baseClasses} font-medium bg-yellow-100 text-yellow-800 rounded-full ${className}`}>
      <svg className={`${inline ? 'w-3 h-3' : 'w-4 h-4'} mr-1`} fill="currentColor" viewBox="0 0 20 20">
        <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clipRule="evenodd" />
      </svg>
      Coming Soon
    </span>
  );
}

interface ComingSoonOverlayProps {
  children: ReactNode;
  message?: string;
  showBadge?: boolean;
}

export function ComingSoonOverlay({ 
  children, 
  message = 'This feature is coming soon!',
  showBadge = true 
}: ComingSoonOverlayProps) {
  return (
    <div className="relative">
      <div className="opacity-50 pointer-events-none select-none">
        {children}
      </div>
      <div className="absolute inset-0 flex flex-col items-center justify-center bg-white/80 backdrop-blur-sm rounded-lg">
        {showBadge && <ComingSoonBadge />}
        <p className="mt-2 text-sm text-gray-600">{message}</p>
      </div>
    </div>
  );
}

interface FeatureCardProps {
  title: string;
  description: string;
  icon?: ReactNode;
  available?: boolean;
  onClick?: () => void;
}

export function FeatureCard({ 
  title, 
  description, 
  icon,
  available = false,
  onClick 
}: FeatureCardProps) {
  const handleClick = () => {
    if (available && onClick) {
      onClick();
    }
  };

  return (
    <div 
      className={`card cursor-pointer transition-all ${
        available 
          ? 'hover:shadow-lg hover:scale-105' 
          : 'opacity-75 cursor-not-allowed'
      }`}
      onClick={handleClick}
    >
      <div className="flex items-start justify-between mb-3">
        <div className="flex items-center gap-3">
          {icon && <div className="text-primary">{icon}</div>}
          <h3 className="text-lg font-semibold">{title}</h3>
        </div>
        {!available && <ComingSoonBadge inline />}
      </div>
      <p className="text-gray-600 text-sm">{description}</p>
    </div>
  );
}

interface PlaceholderSectionProps {
  title: string;
  description?: string;
  icon?: ReactNode;
  height?: string;
}

export function PlaceholderSection({ 
  title, 
  description = 'This feature is currently under development.',
  icon,
  height = '200px' 
}: PlaceholderSectionProps) {
  return (
    <div 
      className="card flex flex-col items-center justify-center text-center"
      style={{ minHeight: height }}
    >
      {icon && (
        <div className="text-gray-400 mb-4">
          {icon}
        </div>
      )}
      <h3 className="text-xl font-semibold text-gray-700 mb-2">{title}</h3>
      <p className="text-gray-500 mb-4">{description}</p>
      <ComingSoonBadge />
    </div>
  );
}
