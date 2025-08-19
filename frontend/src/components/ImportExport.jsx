import React, { useState, useRef } from 'react';
import {
  Upload,
  Download,
  FileText,
  FileSpreadsheet,
  FileCode,
  AlertCircle,
  CheckCircle,
  X,
  File
} from 'lucide-react';
// Import/Export API hooks will be added later
// For now, using mock functions

const ImportExport = ({ deckId, onImportSuccess }) => {
  const [activeTab, setActiveTab] = useState('export');
  const [selectedFormat, setSelectedFormat] = useState('json');
  const [includeProgress, setIncludeProgress] = useState(false);
  const [selectedFile, setSelectedFile] = useState(null);
  const [importFormat, setImportFormat] = useState('json');
  const [validationResult, setValidationResult] = useState(null);
  const [mergeDuplicates, setMergeDuplicates] = useState(false);
  const fileInputRef = useRef(null);

  // Mock API functions for now
  const [isExporting, setIsExporting] = useState(false);
  const [isImporting, setIsImporting] = useState(false);
  const [isValidating, setIsValidating] = useState(false);
  
  const exportDeck = async ({ deckId, format, includeProgress }) => {
    setIsExporting(true);
    try {
      // Call backend API
      const response = await fetch(`/api/v1/import-export/export/${deckId}?format=${format}&include_progress=${includeProgress}`, {
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        }
      });
      if (!response.ok) throw new Error('Export failed');
      return { unwrap: async () => await response.blob() };
    } finally {
      setIsExporting(false);
    }
  };
  
  const importDeck = async (formData) => {
    setIsImporting(true);
    try {
      const response = await fetch('/api/v1/import-export/import', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        },
        body: formData
      });
      const data = await response.json();
      return { unwrap: async () => data };
    } finally {
      setIsImporting(false);
    }
  };
  
  const validateImport = async (formData) => {
    setIsValidating(true);
    try {
      const response = await fetch('/api/v1/import-export/import/validate', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${localStorage.getItem('token')}`
        },
        body: formData
      });
      const data = await response.json();
      return { unwrap: async () => data };
    } finally {
      setIsValidating(false);
    }
  };
  
  const getImportTemplate = async (format) => {
    const response = await fetch(`/api/v1/import-export/templates/${format}`, {
      headers: {
        'Authorization': `Bearer ${localStorage.getItem('token')}`
      }
    });
    return { unwrap: async () => await response.blob() };
  };

  const formatInfo = {
    json: {
      name: 'JSON',
      icon: FileCode,
      description: 'Complete deck data with all metadata',
      color: 'text-green-500'
    },
    csv: {
      name: 'CSV',
      icon: FileSpreadsheet,
      description: 'Simple spreadsheet format',
      color: 'text-blue-500'
    },
    markdown: {
      name: 'Markdown',
      icon: FileText,
      description: 'Human-readable text format',
      color: 'text-purple-500'
    },
    anki: {
      name: 'Anki',
      icon: File,
      description: 'Compatible with Anki flashcard app',
      color: 'text-orange-500'
    }
  };

  const handleExport = async () => {
    try {
      const response = await exportDeck({
        deckId,
        format: selectedFormat,
        includeProgress
      }).unwrap();

      // Create a blob from the response
      const blob = new Blob([response], {
        type: selectedFormat === 'json' ? 'application/json' :
              selectedFormat === 'csv' ? 'text/csv' :
              selectedFormat === 'markdown' ? 'text/markdown' :
              'application/json'
      });

      // Create download link
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `deck_export.${selectedFormat}`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch (error) {
      console.error('Export failed:', error);
    }
  };

  const handleFileSelect = (event) => {
    const file = event.target.files[0];
    if (file) {
      setSelectedFile(file);
      setValidationResult(null);
      
      // Auto-detect format from file extension
      const extension = file.name.split('.').pop().toLowerCase();
      if (['json', 'csv', 'md', 'markdown', 'anki'].includes(extension)) {
        setImportFormat(extension === 'md' ? 'markdown' : extension);
      }
    }
  };

  const handleValidate = async () => {
    if (!selectedFile) return;

    const formData = new FormData();
    formData.append('file', selectedFile);
    formData.append('format', importFormat);

    try {
      const result = await validateImport(formData).unwrap();
      setValidationResult(result);
    } catch (error) {
      console.error('Validation failed:', error);
      setValidationResult({
        is_valid: false,
        errors: [error.message || 'Validation failed'],
        warnings: [],
        deck_count: 0,
        card_count: 0
      });
    }
  };

  const handleImport = async () => {
    if (!selectedFile) return;

    const formData = new FormData();
    formData.append('file', selectedFile);
    formData.append('format', importFormat);
    formData.append('merge_duplicates', mergeDuplicates);

    try {
      const result = await importDeck(formData).unwrap();
      if (result.success) {
        onImportSuccess?.(result);
        setSelectedFile(null);
        setValidationResult(null);
        fileInputRef.current.value = '';
      }
    } catch (error) {
      console.error('Import failed:', error);
    }
  };

  const downloadTemplate = async (format) => {
    try {
      const response = await getImportTemplate(format).unwrap();
      
      const blob = new Blob([response], {
        type: format === 'json' ? 'application/json' :
              format === 'csv' ? 'text/csv' :
              'text/markdown'
      });

      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = `import_template.${format}`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);
    } catch (error) {
      console.error('Template download failed:', error);
    }
  };

  return (
    <div className="import-export-container">
      <div className="tabs">
        <button
          className={`tab ${activeTab === 'export' ? 'active' : ''}`}
          onClick={() => setActiveTab('export')}
        >
          <Download className="icon" />
          Export
        </button>
        <button
          className={`tab ${activeTab === 'import' ? 'active' : ''}`}
          onClick={() => setActiveTab('import')}
        >
          <Upload className="icon" />
          Import
        </button>
      </div>

      {activeTab === 'export' ? (
        <div className="export-section">
          <h3>Export Deck</h3>
          
          <div className="format-selection">
            <label>Select Format:</label>
            <div className="format-options">
              {Object.entries(formatInfo).map(([format, info]) => {
                const Icon = info.icon;
                return (
                  <div
                    key={format}
                    className={`format-option ${selectedFormat === format ? 'selected' : ''}`}
                    onClick={() => setSelectedFormat(format)}
                  >
                    <Icon className={`format-icon ${info.color}`} />
                    <div className="format-details">
                      <span className="format-name">{info.name}</span>
                      <span className="format-description">{info.description}</span>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>

          <div className="export-options">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={includeProgress}
                onChange={(e) => setIncludeProgress(e.target.checked)}
              />
              Include study progress
            </label>
          </div>

          <button
            className="btn btn-primary"
            onClick={handleExport}
            disabled={isExporting || !deckId}
          >
            {isExporting ? 'Exporting...' : 'Export Deck'}
          </button>
        </div>
      ) : (
        <div className="import-section">
          <h3>Import Deck</h3>

          <div className="file-upload-area">
            <input
              ref={fileInputRef}
              type="file"
              accept=".json,.csv,.md,.markdown,.anki"
              onChange={handleFileSelect}
              style={{ display: 'none' }}
            />
            
            {selectedFile ? (
              <div className="selected-file">
                <File className="file-icon" />
                <div className="file-info">
                  <span className="file-name">{selectedFile.name}</span>
                  <span className="file-size">
                    {(selectedFile.size / 1024).toFixed(2)} KB
                  </span>
                </div>
                <button
                  className="remove-file"
                  onClick={() => {
                    setSelectedFile(null);
                    setValidationResult(null);
                    fileInputRef.current.value = '';
                  }}
                >
                  <X />
                </button>
              </div>
            ) : (
              <div
                className="upload-prompt"
                onClick={() => fileInputRef.current.click()}
              >
                <Upload className="upload-icon" />
                <p>Click to select a file or drag and drop</p>
                <p className="file-types">Supports: JSON, CSV, Markdown, Anki</p>
              </div>
            )}
          </div>

          {selectedFile && (
            <>
              <div className="format-selection">
                <label>Import Format:</label>
                <select
                  value={importFormat}
                  onChange={(e) => setImportFormat(e.target.value)}
                >
                  <option value="json">JSON</option>
                  <option value="csv">CSV</option>
                  <option value="markdown">Markdown</option>
                  <option value="anki">Anki</option>
                </select>
              </div>

              <div className="import-options">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={mergeDuplicates}
                    onChange={(e) => setMergeDuplicates(e.target.checked)}
                  />
                  Merge with existing decks if names match
                </label>
              </div>

              <div className="action-buttons">
                <button
                  className="btn btn-secondary"
                  onClick={handleValidate}
                  disabled={isValidating}
                >
                  {isValidating ? 'Validating...' : 'Validate File'}
                </button>
                
                <button
                  className="btn btn-primary"
                  onClick={handleImport}
                  disabled={isImporting || (validationResult && !validationResult.is_valid)}
                >
                  {isImporting ? 'Importing...' : 'Import Deck'}
                </button>
              </div>
            </>
          )}

          {validationResult && (
            <div className={`validation-result ${validationResult.is_valid ? 'valid' : 'invalid'}`}>
              <div className="validation-header">
                {validationResult.is_valid ? (
                  <>
                    <CheckCircle className="icon success" />
                    <span>File is valid</span>
                  </>
                ) : (
                  <>
                    <AlertCircle className="icon error" />
                    <span>Validation issues found</span>
                  </>
                )}
              </div>
              
              <div className="validation-details">
                <p>Decks: {validationResult.deck_count}</p>
                <p>Cards: {validationResult.card_count}</p>
              </div>

              {validationResult.errors.length > 0 && (
                <div className="validation-errors">
                  <h4>Errors:</h4>
                  <ul>
                    {validationResult.errors.map((error, index) => (
                      <li key={index}>{error}</li>
                    ))}
                  </ul>
                </div>
              )}

              {validationResult.warnings.length > 0 && (
                <div className="validation-warnings">
                  <h4>Warnings:</h4>
                  <ul>
                    {validationResult.warnings.map((warning, index) => (
                      <li key={index}>{warning}</li>
                    ))}
                  </ul>
                </div>
              )}
            </div>
          )}

          <div className="template-section">
            <h4>Download Templates</h4>
            <div className="template-buttons">
              {['json', 'csv', 'markdown'].map(format => {
                const info = formatInfo[format];
                const Icon = info.icon;
                return (
                  <button
                    key={format}
                    className="template-btn"
                    onClick={() => downloadTemplate(format)}
                  >
                    <Icon className={`icon ${info.color}`} />
                    {info.name}
                  </button>
                );
              })}
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default ImportExport;
