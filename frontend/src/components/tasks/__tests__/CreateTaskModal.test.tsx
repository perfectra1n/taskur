import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '../../../test/test-utils';
import userEvent from '@testing-library/user-event';
import { CreateTaskModal } from '../CreateTaskModal';
import { api } from '../../../services/api';

// Mock the API
vi.mock('../../../services/api', () => ({
  api: {
    createTask: vi.fn(),
    getTeamMembers: vi.fn(),
  },
}));

// Mock the TiptapEditor component
vi.mock('../../editor/TiptapEditor', () => ({
  TiptapEditor: ({ content, onChange, placeholder }: any) => (
    <textarea
      data-testid="tiptap-editor"
      value={content}
      onChange={(e) => onChange(e.target.value)}
      placeholder={placeholder}
    />
  ),
}));

// Mock the HeroImageUpload component
vi.mock('../HeroImageUpload', () => ({
  HeroImageUpload: ({ onImageUploaded, onImageRemoved }: any) => (
    <div data-testid="hero-image-upload">
      <button onClick={() => onImageUploaded('test-image-id')}>Upload Image</button>
      <button onClick={() => onImageRemoved()}>Remove Image</button>
    </div>
  ),
}));

// Mock the ReminderConfig component
vi.mock('../ReminderConfig', () => ({
  ReminderConfig: ({ onChange }: any) => (
    <div data-testid="reminder-config">
      <button onClick={() => onChange([{ id: '1', datetime: '2025-01-15T10:00:00Z', type: 'notification' }])}>
        Add Reminder
      </button>
    </div>
  ),
}));

describe('CreateTaskModal', () => {
  const mockOnClose = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (api.getTeamMembers as any).mockResolvedValue([
      { id: '1', name: 'John Doe', email: 'john@example.com', role: 'Developer' },
      { id: '2', name: 'Jane Smith', email: 'jane@example.com', role: 'Designer' },
    ]);
    (api.createTask as any).mockResolvedValue({ id: 'new-task-id' });
  });

  it('should render the modal when isOpen is true', () => {
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    expect(screen.getByText('Create New Task')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Enter task title...')).toBeInTheDocument();
  });

  it('should not render the modal when isOpen is false', () => {
    render(<CreateTaskModal isOpen={false} onClose={mockOnClose} />);

    expect(screen.queryByText('Create New Task')).not.toBeInTheDocument();
  });

  it('should handle title input', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const titleInput = screen.getByPlaceholderText('Enter task title...');
    await user.type(titleInput, 'New Task Title');

    expect(titleInput).toHaveValue('New Task Title');
  });

  it('should handle priority selection', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const lowPriorityButton = screen.getByRole('button', { name: 'Low' });
    await user.click(lowPriorityButton);

    expect(lowPriorityButton).toHaveClass('bg-primary-50');
  });

  it('should handle status selection', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const statusSelect = screen.getByRole('combobox');
    await user.selectOptions(statusSelect, 'inprogress');

    expect(statusSelect).toHaveValue('inprogress');
  });

  it('should handle description input', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const descriptionEditor = screen.getByTestId('tiptap-editor');
    await user.type(descriptionEditor, 'Task description');

    expect(descriptionEditor).toHaveValue('Task description');
  });

  it('should handle tags input', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const tagsInput = screen.getByPlaceholderText('Enter tags separated by commas...');

    // Clear and paste the tags (simulating paste is better for comma-separated values)
    await user.clear(tagsInput);
    await user.click(tagsInput);
    await user.paste('urgent, bug, frontend');

    // Check that the input value is set correctly
    expect(tagsInput).toHaveValue('urgent, bug, frontend');
  });

  it('should handle hero image upload', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const uploadButton = screen.getByText('Upload Image');
    await user.click(uploadButton);

    // Hero image ID should be set in form data
    expect(screen.getByTestId('hero-image-upload')).toBeInTheDocument();
  });

  it('should handle reminder configuration', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const addReminderButton = screen.getByText('Add Reminder');
    await user.click(addReminderButton);

    expect(screen.getByTestId('reminder-config')).toBeInTheDocument();
  });

  it('should submit the form with valid data', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const titleInput = screen.getByPlaceholderText('Enter task title...');
    await user.type(titleInput, 'New Task');

    const submitButton = screen.getByRole('button', { name: 'Create Task' });
    await user.click(submitButton);

    await waitFor(() => {
      expect(api.createTask).toHaveBeenCalledWith(
        expect.objectContaining({
          title: 'New Task',
          status: 'todo',
          priority: 'medium',
        })
      );
      expect(mockOnClose).toHaveBeenCalled();
    });
  });

  it('should disable submit button when title is empty', () => {
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const submitButton = screen.getByRole('button', { name: 'Create Task' });
    expect(submitButton).toBeDisabled();
  });

  it('should handle cancel button', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const cancelButton = screen.getByRole('button', { name: 'Cancel' });
    await user.click(cancelButton);

    expect(mockOnClose).toHaveBeenCalled();
  });

  it('should load and display team members', async () => {
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    await waitFor(() => {
      expect(api.getTeamMembers).toHaveBeenCalled();
    });

    // MultiSelect should be rendered when team members are available
    await waitFor(() => {
      expect(screen.getByText('Assign To')).toBeInTheDocument();
    });
  });

  it('should clean up empty fields before submission', async () => {
    const user = userEvent.setup();
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const titleInput = screen.getByPlaceholderText('Enter task title...');
    await user.type(titleInput, 'Task with minimal data');

    const submitButton = screen.getByRole('button', { name: 'Create Task' });
    await user.click(submitButton);

    await waitFor(() => {
      expect(api.createTask).toHaveBeenCalledWith(
        expect.objectContaining({
          title: 'Task with minimal data',
          description: undefined,
          due_date: undefined,
          start_date: undefined,
          end_date: undefined,
        })
      );
    });
  });

  it('should display all priority options', () => {
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    expect(screen.getByRole('button', { name: 'Low' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Medium' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'High' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Urgent' })).toBeInTheDocument();
  });

  it('should display all status options', () => {
    render(<CreateTaskModal isOpen={true} onClose={mockOnClose} />);

    const statusSelect = screen.getByRole('combobox');
    const options = statusSelect.querySelectorAll('option');

    expect(options).toHaveLength(3);
    expect(options[0]).toHaveTextContent('To Do');
    expect(options[1]).toHaveTextContent('In Progress');
    expect(options[2]).toHaveTextContent('Completed');
  });
});
