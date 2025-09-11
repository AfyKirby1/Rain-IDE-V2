import React, { useState, useRef, useEffect } from 'react';
import {
  Box,
  TextField,
  IconButton,
  Typography,
  CircularProgress,
  Chip,
} from '@mui/material';
import {
  Send,
  Person,
  SmartToy,
  Clear,
  Settings,
} from '@mui/icons-material';
import { useAIStore } from '../stores/aiStore';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

interface AIChatPanelProps {
  onOpenModelPicker?: () => void;
}

const AIChatPanel: React.FC<AIChatPanelProps> = ({ onOpenModelPicker }) => {
  const {
    currentModel,
    generateResponse,
    clearConversation,
  } = useAIStore();

  const [messages, setMessages] = useState<Message[]>([]);
  const [inputText, setInputText] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSendMessage = async () => {
    if (!inputText.trim() || !currentModel || isGenerating) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: inputText.trim(),
      timestamp: new Date(),
    };

    setMessages(prev => [...prev, userMessage]);
    setInputText('');
    setIsGenerating(true);

    try {
      const response = await generateResponse(userMessage.content);
      
      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: response,
        timestamp: new Date(),
      };

      setMessages(prev => [...prev, assistantMessage]);
    } catch (error) {
      let errorContent = 'Failed to generate response';
      
      if (error instanceof Error) {
        if (error.message.includes('Model not properly loaded')) {
          errorContent = '⚠️ Model Loading Error: The AI model is not properly loaded. Please try reloading the model or check if the model files are valid.';
        } else if (error.message.includes('No model loaded')) {
          errorContent = '⚠️ No Model Loaded: Please select and load an AI model first before starting a conversation.';
        } else if (error.message.includes('Invalid conversation format')) {
          errorContent = '⚠️ Conversation Error: There was an issue with the conversation format. Please try again.';
        } else {
          errorContent = `⚠️ Error: ${error.message}`;
        }
      }
      
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: errorContent,
        timestamp: new Date(),
      };
      setMessages(prev => [...prev, errorMessage]);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleClearChat = async () => {
    setMessages([]);
    await clearConversation();
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  if (!currentModel) {
    return (
      <Box
        sx={{
          height: '100%',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          flexDirection: 'column',
          p: 3,
        }}
      >
        <Box
          sx={{
            width: 60,
            height: 60,
            borderRadius: 2,
            bgcolor: 'rgba(156, 163, 175, 0.1)',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            mb: 2,
          }}
        >
          <SmartToy sx={{ fontSize: 32, color: '#9ca3af' }} />
        </Box>
        <Typography variant="h6" sx={{ color: '#e2e8f0', mb: 1, textAlign: 'center' }}>
          No Model Loaded
        </Typography>
        <Typography variant="body2" sx={{ color: '#94a3b8', textAlign: 'center' }}>
          Select and load a model to start chatting
        </Typography>
      </Box>
    );
  }

  return (
    <Box
      sx={{
        height: '100%',
        maxHeight: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: '#0f172a',
        overflow: 'hidden',
      }}
    >
      {/* Header */}
      <Box
        sx={{
          p: 2,
          borderBottom: '1px solid #475569',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'space-between',
        }}
      >
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <SmartToy sx={{ mr: 1, color: '#60a5fa' }} />
          <Typography variant="h6" sx={{ color: '#e2e8f0', fontSize: 14, fontWeight: 600 }}>
            AI Chat
          </Typography>
          <Chip
            label={currentModel.name}
            size="small"
            sx={{ 
              ml: 2, 
              bgcolor: 'rgba(96, 165, 250, 0.1)', 
              color: '#60a5fa',
              fontSize: 10,
              height: 20,
            }}
          />
        </Box>
        
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5 }}>
          {onOpenModelPicker && (
            <IconButton
              onClick={onOpenModelPicker}
              size="small"
              sx={{ color: '#94a3b8' }}
              title="Change Model"
            >
              <Settings fontSize="small" />
            </IconButton>
          )}
          <IconButton
            onClick={handleClearChat}
            size="small"
            sx={{ color: '#94a3b8' }}
            disabled={messages.length === 0}
            title="Clear Chat"
          >
            <Clear fontSize="small" />
          </IconButton>
        </Box>
      </Box>

      {/* Messages */}
      <Box
        sx={{
          flex: 1,
          overflow: 'auto',
          p: 2,
          display: 'flex',
          flexDirection: 'column',
          gap: 2,
          minHeight: 0,
        }}
      >
        {messages.length === 0 && (
          <Box sx={{ textAlign: 'center', py: 4 }}>
            <Typography variant="body2" sx={{ color: '#94a3b8' }}>
              Start a conversation with your AI assistant
            </Typography>
          </Box>
        )}

        {messages.map((message) => (
          <Box
            key={message.id}
            sx={{
              display: 'flex',
              alignItems: 'flex-start',
              gap: 1,
              ...(message.role === 'user' && {
                flexDirection: 'row-reverse',
              }),
            }}
          >
            <Box
              sx={{
                width: 28,
                height: 28,
                borderRadius: '50%',
                bgcolor: message.role === 'user' ? '#3b82f6' : '#6b7280',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                flexShrink: 0,
              }}
            >
              {message.role === 'user' ? (
                <Person sx={{ fontSize: 16, color: 'white' }} />
              ) : (
                <SmartToy sx={{ fontSize: 16, color: 'white' }} />
              )}
            </Box>

            <Box
              sx={{
                maxWidth: '80%',
                bgcolor: message.role === 'user' 
                  ? 'rgba(59, 130, 246, 0.1)' 
                  : 'rgba(255, 255, 255, 0.05)',
                border: message.role === 'user'
                  ? '1px solid rgba(59, 130, 246, 0.3)'
                  : '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: 2,
                p: 1.5,
              }}
            >
              <Typography
                variant="body2"
                sx={{
                  color: '#e2e8f0',
                  whiteSpace: 'pre-wrap',
                  wordBreak: 'break-word',
                  fontSize: 13,
                  lineHeight: 1.4,
                }}
              >
                {message.content}
              </Typography>
              <Typography
                variant="caption"
                sx={{
                  color: '#6b7280',
                  display: 'block',
                  mt: 0.5,
                  textAlign: message.role === 'user' ? 'right' : 'left',
                  fontSize: 10,
                }}
              >
                {formatTime(message.timestamp)}
              </Typography>
            </Box>
          </Box>
        ))}

        {isGenerating && (
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
            <Box
              sx={{
                width: 28,
                height: 28,
                borderRadius: '50%',
                bgcolor: '#6b7280',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <SmartToy sx={{ fontSize: 16, color: 'white' }} />
            </Box>
            <Box
              sx={{
                bgcolor: 'rgba(255, 255, 255, 0.05)',
                border: '1px solid rgba(255, 255, 255, 0.1)',
                borderRadius: 2,
                p: 1.5,
                display: 'flex',
                alignItems: 'center',
                gap: 1,
              }}
            >
              <CircularProgress size={14} sx={{ color: '#60a5fa' }} />
              <Typography variant="body2" sx={{ color: '#94a3b8', fontSize: 13 }}>
                Thinking...
              </Typography>
            </Box>
          </Box>
        )}

        <div ref={messagesEndRef} />
      </Box>

      {/* Input */}
      <Box
        sx={{
          p: 2,
          borderTop: '1px solid #475569',
          display: 'flex',
          gap: 1,
        }}
      >
        <TextField
          fullWidth
          multiline
          maxRows={3}
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          onKeyPress={handleKeyPress}
          placeholder="Type your message..."
          disabled={isGenerating}
          size="small"
          sx={{
            '& .MuiOutlinedInput-root': {
              bgcolor: 'rgba(255, 255, 255, 0.05)',
              '& fieldset': {
                borderColor: '#475569',
              },
              '&:hover fieldset': {
                borderColor: '#64748b',
              },
              '&.Mui-focused fieldset': {
                borderColor: '#60a5fa',
              },
            },
            '& .MuiInputBase-input': {
              color: '#e2e8f0',
              fontSize: 13,
            },
            '& .MuiInputBase-input::placeholder': {
              color: '#6b7280',
              opacity: 1,
            },
          }}
        />
        <IconButton
          onClick={handleSendMessage}
          disabled={!inputText.trim() || isGenerating}
          sx={{
            bgcolor: '#3b82f6',
            color: 'white',
            '&:hover': {
              bgcolor: '#2563eb',
            },
            '&.Mui-disabled': {
              bgcolor: '#374151',
              color: '#6b7280',
            },
          }}
        >
          <Send fontSize="small" />
        </IconButton>
      </Box>
    </Box>
  );
};

export default AIChatPanel;
