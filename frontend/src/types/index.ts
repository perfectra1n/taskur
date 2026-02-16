// Re-export all generated types from the Rust backend
export type {
  AddTeamMemberRequest,
  AttachmentResponse,
  AuthResponse,
  Comment,
  CreateCommentRequest,
  CreateListRequest,
  CreateTagRequest,
  CreateTaskRequest,
  CreateTeamMemberRequest,
  CreateTeamRequest,
  JsonValue,
  List,
  LoginRequest,
  Tag,
  Task,
  TaskFilter,
  TaskPriority,
  TaskStatus,
  Team,
  TeamMember,
  TeamMemberWithUser,
  TeamUserAssociation,
  UnifiedSearchQuery,
  UnifiedSearchResult,
  UpdateCommentRequest,
  UpdateListRequest,
  UpdateTaskRequest,
  UpdateTeamMemberRequest,
  UpdateTeamRequest,
  UserResponse,
} from './generated';

// Aliases for names used in the frontend
export type { UserResponse as User } from './generated';
export type { AttachmentResponse as Attachment } from './generated';

// Frontend-only types (not in Rust)
export interface Reminder {
  id: string;
  datetime: string;
  type: 'notification' | 'email';
  message?: string;
}
