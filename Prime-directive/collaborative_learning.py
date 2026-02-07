#!/usr/bin/env python3
"""
Collaborative Learning Module - Real-time Learning Sessions

Provides real-time collaborative features for multi-user physics learning:
- Shared learning sessions
- User interaction tracking
- Collaborative annotations
- Progress sharing
- Discussion threads
- Group knowledge graphs
"""

from typing import Dict, List, Optional, Set
from datetime import datetime
from dataclasses import dataclass, asdict
from enum import Enum
import json
import uuid


class UserRole(Enum):
    """User roles in collaborative sessions."""
    INSTRUCTOR = "instructor"
    STUDENT = "student"
    PEER_TUTOR = "peer_tutor"
    OBSERVER = "observer"


class AnnotationType(Enum):
    """Types of annotations in collaborative learning."""
    QUESTION = "question"
    HIGHLIGHT = "highlight"
    CORRECTION = "correction"
    EXPLANATION = "explanation"
    RESOURCE = "resource"
    BOOKMARK = "bookmark"


@dataclass
class User:
    """Collaborative learning user."""
    user_id: str
    name: str
    role: UserRole
    language: str
    joined_at: str
    last_active: str

    def to_dict(self):
        return {
            'user_id': self.user_id,
            'name': self.name,
            'role': self.role.value,
            'language': self.language,
            'joined_at': self.joined_at,
            'last_active': self.last_active
        }


@dataclass
class Annotation:
    """User annotation on learning content."""
    annotation_id: str
    user_id: str
    content_id: str
    type: AnnotationType
    text: str
    start_position: Optional[int]
    end_position: Optional[int]
    created_at: str
    replies: List[str]

    def to_dict(self):
        return {
            'annotation_id': self.annotation_id,
            'user_id': self.user_id,
            'content_id': self.content_id,
            'type': self.type.value,
            'text': self.text,
            'start_position': self.start_position,
            'end_position': self.end_position,
            'created_at': self.created_at,
            'reply_count': len(self.replies)
        }


@dataclass
class LearningSession:
    """Collaborative learning session."""
    session_id: str
    title: str
    topic: str
    language: str
    created_at: str
    created_by: str
    active: bool
    max_participants: int

    def to_dict(self):
        return {
            'session_id': self.session_id,
            'title': self.title,
            'topic': self.topic,
            'language': self.language,
            'created_at': self.created_at,
            'created_by': self.created_by,
            'active': self.active,
            'max_participants': self.max_participants
        }


class CollaborativeLearningManager:
    """
    Manages real-time collaborative learning sessions.
    Handles user interactions, annotations, and group activities.
    """

    def __init__(self):
        """Initialize collaborative learning manager."""
        self.sessions: Dict[str, LearningSession] = {}
        self.session_users: Dict[str, Dict[str, User]] = {}  # session_id -> {user_id -> User}
        self.session_annotations: Dict[str, List[Annotation]] = {}
        self.session_history: Dict[str, List[Dict]] = {}
        self.user_progress: Dict[str, Dict] = {}  # session_id -> {user_id -> progress_data}

    def create_session(self, title: str, topic: str, language: str, creator_id: str,
                      max_participants: int = 50) -> Dict:
        """
        Create a new collaborative learning session.

        Args:
            title: Session title
            topic: Physics topic
            language: Session language
            creator_id: Creator user ID
            max_participants: Maximum participants

        Returns:
            Session information
        """
        session_id = str(uuid.uuid4())
        now = datetime.now().isoformat()

        session = LearningSession(
            session_id=session_id,
            title=title,
            topic=topic,
            language=language,
            created_at=now,
            created_by=creator_id,
            active=True,
            max_participants=max_participants
        )

        self.sessions[session_id] = session
        self.session_users[session_id] = {}
        self.session_annotations[session_id] = []
        self.session_history[session_id] = []
        self.user_progress[session_id] = {}

        self._log_event(session_id, 'session_created', {
            'creator': creator_id,
            'title': title,
            'topic': topic
        })

        return {
            'success': True,
            'session': session.to_dict(),
            'message': f'Session "{title}" created successfully'
        }

    def add_user(self, session_id: str, user_id: str, name: str, role: UserRole,
                language: str) -> Dict:
        """
        Add user to a collaborative session.

        Args:
            session_id: Session ID
            user_id: User ID
            name: User name
            role: User role
            language: User's preferred language

        Returns:
            Operation result
        """
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        session = self.sessions[session_id]

        if len(self.session_users[session_id]) >= session.max_participants:
            return {'success': False, 'error': 'Session is full'}

        if user_id in self.session_users[session_id]:
            return {'success': False, 'error': 'User already in session'}

        now = datetime.now().isoformat()
        user = User(
            user_id=user_id,
            name=name,
            role=role,
            language=language,
            joined_at=now,
            last_active=now
        )

        self.session_users[session_id][user_id] = user
        self.user_progress[session_id][user_id] = {
            'user_id': user_id,
            'annotations_created': 0,
            'concepts_learned': [],
            'time_spent_seconds': 0
        }

        self._log_event(session_id, 'user_joined', {
            'user_id': user_id,
            'name': name,
            'role': role.value
        })

        return {
            'success': True,
            'user': user.to_dict(),
            'participant_count': len(self.session_users[session_id]),
            'message': f'{name} joined the session'
        }

    def add_annotation(self, session_id: str, user_id: str, content_id: str,
                      annotation_type: AnnotationType, text: str,
                      start_pos: Optional[int] = None,
                      end_pos: Optional[int] = None) -> Dict:
        """
        Add an annotation to learning content.

        Args:
            session_id: Session ID
            user_id: User ID
            content_id: Content being annotated
            annotation_type: Type of annotation
            text: Annotation text
            start_pos: Start position in content
            end_pos: End position in content

        Returns:
            Annotation information
        """
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        if user_id not in self.session_users[session_id]:
            return {'success': False, 'error': 'User not in session'}

        annotation_id = str(uuid.uuid4())
        now = datetime.now().isoformat()

        annotation = Annotation(
            annotation_id=annotation_id,
            user_id=user_id,
            content_id=content_id,
            type=annotation_type,
            text=text,
            start_position=start_pos,
            end_position=end_pos,
            created_at=now,
            replies=[]
        )

        self.session_annotations[session_id].append(annotation)
        self.user_progress[session_id][user_id]['annotations_created'] += 1

        self._log_event(session_id, 'annotation_created', {
            'user_id': user_id,
            'type': annotation_type.value,
            'content': content_id
        })

        return {
            'success': True,
            'annotation': annotation.to_dict(),
            'message': f'Annotation created and shared with {len(self.session_users[session_id])} participants'
        }

    def reply_to_annotation(self, session_id: str, user_id: str,
                           annotation_id: str, reply_text: str) -> Dict:
        """
        Reply to an existing annotation (discussion thread).

        Args:
            session_id: Session ID
            user_id: Replying user ID
            annotation_id: Annotation to reply to
            reply_text: Reply text

        Returns:
            Operation result
        """
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        annotations = self.session_annotations[session_id]
        annotation = next((a for a in annotations if a.annotation_id == annotation_id), None)

        if not annotation:
            return {'success': False, 'error': 'Annotation not found'}

        reply_id = str(uuid.uuid4())
        annotation.replies.append(reply_id)

        self._log_event(session_id, 'reply_created', {
            'user_id': user_id,
            'annotation_id': annotation_id,
            'reply_id': reply_id
        })

        return {
            'success': True,
            'reply_id': reply_id,
            'annotation_id': annotation_id,
            'total_replies': len(annotation.replies),
            'message': 'Reply shared with discussion thread'
        }

    def record_concept_learning(self, session_id: str, user_id: str,
                               concept: str, confidence: float) -> Dict:
        """
        Record that a user has learned a concept.

        Args:
            session_id: Session ID
            user_id: User ID
            concept: Physics concept learned
            confidence: Learning confidence (0-1)

        Returns:
            Operation result
        """
        if session_id not in self.sessions or user_id not in self.session_users[session_id]:
            return {'success': False, 'error': 'Invalid session or user'}

        progress = self.user_progress[session_id][user_id]
        progress['concepts_learned'].append({
            'concept': concept,
            'confidence': confidence,
            'learned_at': datetime.now().isoformat()
        })

        self._log_event(session_id, 'concept_learned', {
            'user_id': user_id,
            'concept': concept,
            'confidence': confidence
        })

        return {
            'success': True,
            'concept': concept,
            'concepts_learned_count': len(progress['concepts_learned']),
            'message': f'Concept "{concept}" recorded as learned'
        }

    def get_session_stats(self, session_id: str) -> Dict:
        """Get statistics for a collaborative session."""
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        session = self.sessions[session_id]
        users = self.session_users[session_id]
        annotations = self.session_annotations[session_id]
        progress = self.user_progress[session_id]

        # Calculate stats
        total_concepts = sum(len(p.get('concepts_learned', []))
                           for p in progress.values())
        avg_confidence = 0
        if total_concepts > 0:
            all_confidences = [c['confidence'] for p in progress.values()
                              for c in p.get('concepts_learned', [])]
            avg_confidence = sum(all_confidences) / len(all_confidences)

        annotation_types = {}
        for ann in annotations:
            type_name = ann.type.value
            annotation_types[type_name] = annotation_types.get(type_name, 0) + 1

        return {
            'success': True,
            'session': session.to_dict(),
            'statistics': {
                'participant_count': len(users),
                'total_annotations': len(annotations),
                'annotation_types': annotation_types,
                'total_concepts_learned': total_concepts,
                'average_confidence': round(avg_confidence, 3),
                'user_roles': {role.value: sum(1 for u in users.values()
                                              if u.role == role)
                             for role in UserRole},
                'active_discussions': sum(len(a.replies) for a in annotations)
            }
        }

    def get_user_progress(self, session_id: str, user_id: str) -> Dict:
        """Get individual user progress in session."""
        if session_id not in self.sessions or user_id not in self.session_users[session_id]:
            return {'success': False, 'error': 'Invalid session or user'}

        user = self.session_users[session_id][user_id]
        progress = self.user_progress[session_id][user_id]
        user_annotations = [a for a in self.session_annotations[session_id]
                           if a.user_id == user_id]

        return {
            'success': True,
            'user': user.to_dict(),
            'progress': {
                **progress,
                'annotations_created': len(user_annotations),
                'discussion_replies': sum(len(a.replies) for a in user_annotations)
            }
        }

    def get_knowledge_graph(self, session_id: str, language: str = 'en') -> Dict:
        """
        Generate a knowledge graph showing concept relationships
        and learning pathways in the session.
        """
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        progress = self.user_progress[session_id]

        # Collect all concepts
        all_concepts = []
        for user_progress in progress.values():
            for concept_data in user_progress.get('concepts_learned', []):
                all_concepts.append(concept_data['concept'])

        # Create concept relationships (simplified)
        concept_pairs = {
            'gravity': ['force', 'mass', 'acceleration'],
            'entropy': ['disorder', 'heat', 'thermodynamics'],
            'light': ['wave', 'photon', 'electromagnetic'],
            'energy': ['work', 'power', 'conservation'],
            'wave': ['frequency', 'wavelength', 'oscillation']
        }

        nodes = list(set(all_concepts))
        edges = []

        for concept in nodes:
            related = concept_pairs.get(concept.lower(), [])
            for rel in related:
                if rel in nodes:
                    edges.append({'source': concept, 'target': rel})

        return {
            'success': True,
            'knowledge_graph': {
                'nodes': nodes,
                'edges': edges,
                'total_concepts': len(nodes),
                'total_relationships': len(edges)
            },
            'language': language
        }

    def _log_event(self, session_id: str, event_type: str, data: Dict) -> None:
        """Log an event in session history."""
        event = {
            'timestamp': datetime.now().isoformat(),
            'type': event_type,
            'data': data
        }
        self.session_history[session_id].append(event)

    def export_session_data(self, session_id: str) -> Dict:
        """Export complete session data for analysis."""
        if session_id not in self.sessions:
            return {'success': False, 'error': 'Session not found'}

        return {
            'success': True,
            'session': self.sessions[session_id].to_dict(),
            'users': [u.to_dict() for u in self.session_users[session_id].values()],
            'annotations': [a.to_dict() for a in self.session_annotations[session_id]],
            'progress': self.user_progress[session_id],
            'history': self.session_history[session_id],
            'export_timestamp': datetime.now().isoformat()
        }


if __name__ == "__main__":
    # Test collaborative learning system
    manager = CollaborativeLearningManager()

    print("Testing Collaborative Learning System\n")
    print("="*70)

    # Create a session
    print("\n✓ Creating collaborative learning session...")
    session_result = manager.create_session(
        title="Quantum Mechanics Fundamentals",
        topic="quantum_mechanics",
        language="en",
        creator_id="instructor_001"
    )
    session_id = session_result['session']['session_id']
    print(f"  Session ID: {session_id}")

    # Add users
    print("\n✓ Adding collaborative learners...")
    manager.add_user(session_id, "student_001", "Alice", UserRole.STUDENT, "en")
    manager.add_user(session_id, "student_002", "Bob", UserRole.STUDENT, "en")
    manager.add_user(session_id, "tutor_001", "Charlie", UserRole.PEER_TUTOR, "en")

    # Add annotations
    print("\n✓ Creating collaborative annotations...")
    ann1 = manager.add_annotation(
        session_id, "student_001", "superposition",
        AnnotationType.QUESTION, "What does superposition really mean?"
    )
    print(f"  Question annotation created")

    ann2 = manager.add_annotation(
        session_id, "tutor_001", "superposition",
        AnnotationType.EXPLANATION, "Superposition means the particle can be in multiple states simultaneously..."
    )
    print(f"  Explanation annotation created")

    # Record concept learning
    print("\n✓ Recording concept mastery...")
    manager.record_concept_learning(session_id, "student_001", "superposition", 0.85)
    manager.record_concept_learning(session_id, "student_001", "entanglement", 0.75)
    manager.record_concept_learning(session_id, "student_002", "superposition", 0.90)

    # Get session stats
    print("\n✓ Session Statistics:")
    stats = manager.get_session_stats(session_id)
    stats_data = stats['statistics']
    print(f"  Participants: {stats_data['participant_count']}")
    print(f"  Total Annotations: {stats_data['total_annotations']}")
    print(f"  Concepts Learned: {stats_data['total_concepts_learned']}")
    print(f"  Average Confidence: {stats_data['average_confidence']}")

    # Get knowledge graph
    print("\n✓ Knowledge Graph:")
    kg = manager.get_knowledge_graph(session_id)
    kg_data = kg['knowledge_graph']
    print(f"  Concepts: {kg_data['total_concepts']}")
    print(f"  Relationships: {kg_data['total_relationships']}")

    print("\n" + "="*70)
