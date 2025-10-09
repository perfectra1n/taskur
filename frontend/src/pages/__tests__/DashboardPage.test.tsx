import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '../../test/test-utils';
import userEvent from '@testing-library/user-event';
import { DashboardPage } from '../DashboardPage';
import { api } from '../../services/api';
import { useAuthStore } from '../../stores/authStore';

// Mock the API
vi.mock('../../services/api', () => ({
  api: {
    getTasks: vi.fn(),
    getTeamMembers: vi.fn(),
  },
}));

// Create a mock logout function at module level
const mockLogoutFn = vi.fn();

// Mock the auth store
vi.mock('../../stores/authStore', () => ({
  useAuthStore: vi.fn((selector) => {
    const state = {
      user: { email: 'test@example.com' },
      logout: mockLogoutFn,
    };
    return selector ? selector(state) : state;
  }),
}));

// Mock child components
vi.mock('../../components/tasks/TaskList', () => ({
  TaskList: ({ tasks, onTaskClick }: any) => (
    <div data-testid="task-list">
      {tasks.map((task: any) => (
        <div key={task.id} onClick={() => onTaskClick(task)}>
          {task.title}
        </div>
      ))}
    </div>
  ),
}));

vi.mock('../../components/tasks/TaskDetail', () => ({
  TaskDetail: ({ task, onClose }: any) => (
    <div data-testid="task-detail">
      <h2>{task.title}</h2>
      <button onClick={onClose}>Close</button>
    </div>
  ),
}));

vi.mock('../../components/tasks/CreateTaskModal', () => ({
  CreateTaskModal: ({ isOpen, onClose }: any) =>
    isOpen ? (
      <div data-testid="create-task-modal">
        <h2>Create New Task</h2>
        <button onClick={onClose}>Close Modal</button>
      </div>
    ) : null,
}));

const mockTasks = [
  {
    id: '1',
    title: 'Test Task 1',
    description: 'Description 1',
    status: 'todo' as const,
    priority: 'medium' as const,
    user_id: 'user-1',
    due_date: null,
    start_date: null,
    end_date: null,
    hero_image_id: null,
    assigned_to: [],
    reminders: [],
    tags: [],
    position: 1,
    created_at: '2025-01-01T00:00:00Z',
    updated_at: '2025-01-01T00:00:00Z',
  },
  {
    id: '2',
    title: 'Test Task 2',
    description: 'Description 2',
    status: 'inprogress' as const,
    priority: 'high' as const,
    user_id: 'user-1',
    due_date: null,
    start_date: null,
    end_date: null,
    hero_image_id: null,
    assigned_to: [],
    reminders: [],
    tags: [],
    position: 2,
    created_at: '2025-01-01T00:00:00Z',
    updated_at: '2025-01-01T00:00:00Z',
  },
];

describe('DashboardPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (api.getTasks as any).mockResolvedValue(mockTasks);
    (api.getTeamMembers as any).mockResolvedValue([]);
  });

  it('should render the dashboard with tasks', async () => {
    render(<DashboardPage />);

    expect(screen.getByText('Taskur')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search tasks... (⌘K)')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByTestId('task-list')).toBeInTheDocument();
    });
  });

  it('should display user email in header', async () => {
    render(<DashboardPage />);

    await waitFor(() => {
      expect(screen.getByText('test@example.com')).toBeInTheDocument();
    });
  });

  it('should open create task modal when New Task button is clicked', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    // Get all buttons and find the New Task button in the header
    const newTaskButtons = screen.getAllByRole('button', { name: /New Task/i });
    await user.click(newTaskButtons[0]); // Click the first one (header button)

    await waitFor(() => {
      expect(screen.getByTestId('create-task-modal')).toBeInTheDocument();
    });
  });

  it('should open create task modal with Ctrl+K keyboard shortcut', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    // Press Ctrl+K
    await user.keyboard('{Control>}k{/Control}');

    await waitFor(() => {
      expect(screen.getByTestId('create-task-modal')).toBeInTheDocument();
    });
  });

  it('should open create task modal with Cmd+K keyboard shortcut', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    // Press Cmd+K (Meta+K)
    await user.keyboard('{Meta>}k{/Meta}');

    await waitFor(() => {
      expect(screen.getByTestId('create-task-modal')).toBeInTheDocument();
    });
  });

  it('should close create task modal with Escape key', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    // Open modal
    await user.keyboard('{Control>}k{/Control}');

    await waitFor(() => {
      expect(screen.getByTestId('create-task-modal')).toBeInTheDocument();
    });

    // Close with Escape
    await user.keyboard('{Escape}');

    await waitFor(() => {
      expect(screen.queryByTestId('create-task-modal')).not.toBeInTheDocument();
    });
  });

  it('should handle search query', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    const searchInput = screen.getByPlaceholderText('Search tasks... (⌘K)');
    await user.type(searchInput, 'Test Task');

    expect(searchInput).toHaveValue('Test Task');

    // Verify API was called with search query
    await waitFor(() => {
      expect(api.getTasks).toHaveBeenCalledWith({ search: 'Test Task' });
    });
  });

  it('should handle logout', async () => {
    // Mock window.location
    Object.defineProperty(window, 'location', {
      writable: true,
      value: { href: '' },
    });

    const user = userEvent.setup();
    render(<DashboardPage />);

    // Find logout button by looking for LogOut icon's parent button
    const buttons = screen.getAllByRole('button');
    const logoutButton = buttons.find(button => button.querySelector('.lucide-log-out'));

    expect(logoutButton).toBeDefined();

    if (logoutButton) {
      await user.click(logoutButton);
      expect(mockLogoutFn).toHaveBeenCalled();
    }
  });

  it('should display task detail when task is clicked', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    await waitFor(() => {
      expect(screen.getByText('Test Task 1')).toBeInTheDocument();
    });

    // Click on task
    await user.click(screen.getByText('Test Task 1'));

    await waitFor(() => {
      expect(screen.getByTestId('task-detail')).toBeInTheDocument();
    });
  });

  it('should close task detail with Escape key when task is selected', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    await waitFor(() => {
      expect(screen.getByText('Test Task 1')).toBeInTheDocument();
    });

    // Click on task to open detail
    await user.click(screen.getByText('Test Task 1'));

    await waitFor(() => {
      expect(screen.getByTestId('task-detail')).toBeInTheDocument();
    });

    // Press Escape to close
    await user.keyboard('{Escape}');

    await waitFor(() => {
      expect(screen.queryByTestId('task-detail')).not.toBeInTheDocument();
    });
  });

  it('should prioritize closing modal over task detail when both are open and Escape is pressed', async () => {
    const user = userEvent.setup();
    render(<DashboardPage />);

    // Open task detail
    await waitFor(() => {
      expect(screen.getByText('Test Task 1')).toBeInTheDocument();
    });
    await user.click(screen.getByText('Test Task 1'));

    await waitFor(() => {
      expect(screen.getByTestId('task-detail')).toBeInTheDocument();
    });

    // Open create modal
    await user.keyboard('{Control>}k{/Control}');

    await waitFor(() => {
      expect(screen.getByTestId('create-task-modal')).toBeInTheDocument();
    });

    // Press Escape - should close modal first, not task detail
    await user.keyboard('{Escape}');

    await waitFor(() => {
      expect(screen.queryByTestId('create-task-modal')).not.toBeInTheDocument();
      expect(screen.getByTestId('task-detail')).toBeInTheDocument(); // Task detail should still be open
    });
  });

  it('should display loading state while fetching tasks', () => {
    (api.getTasks as any).mockImplementation(() => new Promise(() => {})); // Never resolves

    render(<DashboardPage />);

    expect(screen.getByText('Loading tasks...')).toBeInTheDocument();
  });

  it('should call getTasks without search parameter when search is empty', async () => {
    render(<DashboardPage />);

    await waitFor(() => {
      expect(api.getTasks).toHaveBeenCalledWith(undefined);
    });
  });
});
