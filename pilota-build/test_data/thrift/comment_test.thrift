// This is a test file for comment preservation functionality

// User represents a user in the system
// It contains basic user information
struct User {
    // Unique identifier for the user
    1: required i64 id,
    
    // User's display name
    2: optional string name,
    
    // User's email address
    3: optional string email,
}

// Status enumeration for user account
enum UserStatus {
    // Account is active and can be used
    ACTIVE = 1,
    
    // Account is temporarily suspended
    SUSPENDED = 2,
    
    // Account is permanently disabled
    DISABLED = 3,
}

// Service for managing users
service UserService {
    // Get user information by ID
    User getUser(1: i64 userId),
    
    // Create a new user account
    User createUser(1: User user),
    
    // Update user status
    void updateUserStatus(1: i64 userId, 2: UserStatus status),
}