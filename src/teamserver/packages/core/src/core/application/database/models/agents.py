# Agents models. This includes the agents and their associated data.
# this includes things like active callbacks and host information.

from ..db import Base
from sqlalchemy import Column, Integer, String, DateTime, ForeignKey, Enum
from datetime import datetime
from sqlalchemy.orm import relationship





class Agent(Base):
    __tablename__ = "agents"
    id = Column(Integer, primary_key=True, index=True)
    name = Column(String, unique=True, index=True)
    created_at = Column(DateTime, default=datetime.now)
    updated_at = Column(DateTime, default=datetime.now, onupdate=datetime.now)
    host_id = Column(Integer, ForeignKey("hosts.id"))
    host = relationship("Host", back_populates="agents")
    callbacks = relationship("Callback", back_populates="agent")
    objectives = relationship("Objective", back_populates="agents")
    goals = relationship("Goal", back_populates="agents")
    assigned_tasks = relationship("Task", back_populates="agents")
    completed_tasks = relationship("TaskResult", back_populates="agents")
    