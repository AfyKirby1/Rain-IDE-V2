import React, { useState } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Paper,
  Alert,
  CircularProgress,
  Chip,
  Grid,
} from '@mui/material';
import { invoke } from '@tauri-apps/api/core';

interface EmbeddingResult {
  embeddings: number[];
  dimension: number;
  model_name: string;
}

interface SimilarityResult {
  similarities: number[];
  ranked_documents: Array<[number, number]>;
}

const EmbeddingTestPanel: React.FC = () => {
  const [inputText, setInputText] = useState('');
  const [taskType, setTaskType] = useState('query');
  const [documents, setDocuments] = useState(['Document 1', 'Document 2', 'Document 3']);
  const [embeddingResult, setEmbeddingResult] = useState<EmbeddingResult | null>(null);
  const [similarityResult, setSimilarityResult] = useState<SimilarityResult | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const handleEncodeText = async () => {
    if (!inputText.trim()) return;
    
    setIsLoading(true);
    try {
      const result = await invoke('encode_text', {
        request: {
          text: inputText,
          task_type: taskType,
        },
      }) as EmbeddingResult;
      
      setEmbeddingResult(result);
    } catch (error) {
      console.error('Failed to encode text:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleComputeSimilarity = async () => {
    if (!inputText.trim() || documents.length === 0) return;
    
    setIsLoading(true);
    try {
      const result = await invoke('compute_similarity', {
        request: {
          query: inputText,
          documents: documents.filter(d => d.trim()),
        },
      }) as SimilarityResult;
      
      setSimilarityResult(result);
    } catch (error) {
      console.error('Failed to compute similarity:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleLoadModel = async () => {
    try {
      await invoke('load_embedding_model', { modelName: 'google/embeddinggemma-300m' });
      console.log('Model loaded successfully');
    } catch (error) {
      console.error('Failed to load model:', error);
    }
  };

  return (
    <Paper sx={{ p: 3, mb: 3 }}>
      <Typography variant="h6" sx={{ mb: 2, display: 'flex', alignItems: 'center' }}>
        🧠 EmbeddingGemma-300m Test Panel
      </Typography>

      {/* Model Status */}
      <Alert severity="info" sx={{ mb: 2 }}>
        This is a test panel for EmbeddingGemma-300m integration. 
        The actual model download and loading will be implemented in the next phase.
      </Alert>

      {/* Load Model Button */}
      <Box sx={{ mb: 3 }}>
        <Button
          variant="contained"
          onClick={handleLoadModel}
          sx={{ mr: 2 }}
        >
          Load EmbeddingGemma Model
        </Button>
        <Typography variant="caption" sx={{ color: 'text.secondary' }}>
          This will prepare the model for embedding operations
        </Typography>
      </Box>

      <Grid container spacing={3}>
        {/* Text Input Section */}
        <Grid item xs={12} md={6}>
          <Typography variant="subtitle1" sx={{ mb: 2 }}>
            Text Input
          </Typography>
          
          <TextField
            fullWidth
            multiline
            rows={4}
            label="Text to encode"
            value={inputText}
            onChange={(e) => setInputText(e.target.value)}
            placeholder="Enter text to convert to embeddings..."
            sx={{ mb: 2 }}
          />

          <TextField
            select
            fullWidth
            label="Task Type"
            value={taskType}
            onChange={(e) => setTaskType(e.target.value)}
            sx={{ mb: 2 }}
            SelectProps={{ native: true }}
          >
            <option value="query">Query (Search)</option>
            <option value="document">Document</option>
            <option value="classification">Classification</option>
            <option value="clustering">Clustering</option>
            <option value="similarity">Similarity</option>
            <option value="code_retrieval">Code Retrieval</option>
          </TextField>

          <Box sx={{ display: 'flex', gap: 1, mb: 2 }}>
            <Button
              variant="contained"
              onClick={handleEncodeText}
              disabled={!inputText.trim() || isLoading}
              startIcon={isLoading ? <CircularProgress size={16} /> : null}
            >
              Encode Text
            </Button>
            
            <Button
              variant="outlined"
              onClick={handleComputeSimilarity}
              disabled={!inputText.trim() || isLoading}
            >
              Compute Similarity
            </Button>
          </Box>
        </Grid>

        {/* Documents Section */}
        <Grid item xs={12} md={6}>
          <Typography variant="subtitle1" sx={{ mb: 2 }}>
            Documents for Similarity
          </Typography>
          
          {documents.map((doc, index) => (
            <TextField
              key={index}
              fullWidth
              size="small"
              value={doc}
              onChange={(e) => {
                const newDocs = [...documents];
                newDocs[index] = e.target.value;
                setDocuments(newDocs);
              }}
              placeholder={`Document ${index + 1}`}
              sx={{ mb: 1 }}
            />
          ))}
          
          <Button
            size="small"
            onClick={() => setDocuments([...documents, ''])}
            sx={{ mt: 1 }}
          >
            Add Document
          </Button>
        </Grid>
      </Grid>

      {/* Results Section */}
      {embeddingResult && (
        <Box sx={{ mt: 3 }}>
          <Typography variant="subtitle1" sx={{ mb: 2 }}>
            Embedding Results
          </Typography>
          
          <Alert severity="success" sx={{ mb: 2 }}>
            Generated {embeddingResult.dimension}-dimensional embedding using {embeddingResult.model_name}
          </Alert>
          
          <Box sx={{ 
            p: 2, 
            bgcolor: 'grey.100', 
            borderRadius: 1,
            fontFamily: 'monospace',
            fontSize: '0.8rem',
            maxHeight: 200,
            overflow: 'auto'
          }}>
            <Typography variant="body2">
              [{embeddingResult.embeddings.slice(0, 20).map(n => n.toFixed(4)).join(', ')}
              {embeddingResult.embeddings.length > 20 ? '...' : ''}]
            </Typography>
            <Typography variant="caption" sx={{ color: 'text.secondary' }}>
              Showing first 20 dimensions of {embeddingResult.embeddings.length} total
            </Typography>
          </Box>
        </Box>
      )}

      {similarityResult && (
        <Box sx={{ mt: 3 }}>
          <Typography variant="subtitle1" sx={{ mb: 2 }}>
            Similarity Results
          </Typography>
          
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1 }}>
            {similarityResult.similarities.map((similarity, index) => (
              <Chip
                key={index}
                label={`Doc ${index + 1}: ${(similarity * 100).toFixed(1)}%`}
                color={
                  similarity > 0.7 ? 'success' : 
                  similarity > 0.5 ? 'warning' : 
                  'default'
                }
                variant="outlined"
              />
            ))}
          </Box>
          
          <Typography variant="body2" sx={{ mt: 2, color: 'text.secondary' }}>
            Ranked by similarity: {similarityResult.ranked_documents
              .map(([index, score]) => `Doc ${index + 1} (${(score * 100).toFixed(1)}%)`)
              .join(', ')}
          </Typography>
        </Box>
      )}

      {/* Instructions */}
      <Box sx={{ mt: 3, p: 2, bgcolor: 'info.light', borderRadius: 1 }}>
        <Typography variant="body2" sx={{ fontWeight: 'bold', mb: 1 }}>
          How to use:
        </Typography>
        <Typography variant="body2" component="div">
          1. <strong>Load Model</strong>: Click "Load EmbeddingGemma Model" to initialize
          <br />
          2. <strong>Encode Text</strong>: Enter text and select task type, then click "Encode Text"
          <br />
          3. <strong>Similarity Search</strong>: Enter a query and documents, then click "Compute Similarity"
          <br />
          4. <strong>Task Types</strong>: Choose the appropriate task type for better embeddings
        </Typography>
      </Box>
    </Paper>
  );
};

export default EmbeddingTestPanel;
