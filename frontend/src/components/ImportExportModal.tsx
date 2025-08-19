import React from 'react';
import { X, Upload, Download } from 'lucide-react';
import ImportExport from './ImportExport';

interface ImportExportModalProps {
  isOpen: boolean;
  onClose: () => void;
  deckId?: string;
  deckName?: string;
}

const ImportExportModal: React.FC<ImportExportModalProps> = ({
  isOpen,
  onClose,
  deckId,
  deckName,
}) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] overflow-y-auto">
        {/* Modal Header */}
        <div className="flex justify-between items-center p-6 border-b border-gray-200">
          <h2 className="text-2xl font-bold text-[rgb(18_55_64)]">
            Import/Export - {deckName || 'Deck'}
          </h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
            aria-label="Close modal"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Modal Body */}
        <div className="p-6">
          <ImportExport deckId={deckId} />
        </div>
      </div>
    </div>
  );
};

export default ImportExportModal;
