# Objectives models. THis includes the objectives and goals of different campaigns.

from ..db import Base
from sqlalchemy import Column, Integer, String, DateTime, ForeignKey, Enum
from datetime import datetime
from sqlalchemy.orm import relationship

class Campaign(Base):
    __tablename__ = "campaigns"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)


class Objective(Base):
    __tablename__ = "objectives"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)
    campaign_id = Column(Integer, ForeignKey("campaigns.id"))
    campaign = relationship("Campaign", back_populates="objectives")
    agents = relationship("Agent", back_populates="objectives")
    goals = relationship("Goal", back_populates="objectives")


class Goal(Base):
    __tablename__ = "goals"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)
    objective_id = Column(Integer, ForeignKey("objectives.id"))
    objective = relationship("Objective", back_populates="goals")
    agents = relationship("Agent", back_populates="goals")
    tasks = relationship("Task", back_populates="goals")
    task_results = relationship("TaskResult", back_populates="goals")


class Task(Base):
    __tablename__ = "tasks"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    description = Column(String)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)
    goal_id = Column(Integer, ForeignKey("goals.id"))
    goal = relationship("Goal", back_populates="tasks")
    agents = relationship("Agent", back_populates="tasks")
    task_results = relationship("TaskResult", back_populates="tasks")