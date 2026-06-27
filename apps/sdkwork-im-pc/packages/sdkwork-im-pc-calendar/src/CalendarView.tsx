import React, { useState, useEffect } from 'react';
import { CalendarSidebar } from './CalendarSidebar';
import { CalendarContent } from './CalendarContent';
import { CreateEventModal } from './CreateEventModal';
import { calendarService, CalendarEvent } from './services/CalendarService';

export const CalendarView: React.FC = () => {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [editingEvent, setEditingEvent] = useState<CalendarEvent | null>(null);
  const [events, setEvents] = useState<CalendarEvent[]>([]);

  const refreshEvents = () => {
    calendarService.getEvents('', '').then(fetchedEvents => {
      setEvents(fetchedEvents);
    });
  };

  useEffect(() => {
    refreshEvents();
  }, []);

  const handleSaveEvent = async (eventData: CalendarEvent) => {
    try {
      if (editingEvent) {
        const updatedEvent = await calendarService.updateEvent(editingEvent.id, eventData);
        setEvents(events.map(e => e.id === editingEvent.id ? updatedEvent : e));
      } else {
        const createdEvent = await calendarService.createEvent(eventData);
        setEvents([...events, createdEvent]);
      }
      setIsCreateModalOpen(false);
      setEditingEvent(null);
    } catch (err) {
      console.error(err);
    }
  };

  const handleEditEvent = (event: CalendarEvent) => {
    setEditingEvent(event);
    setIsCreateModalOpen(true);
  };

  const handleDeleteEvent = async (id: string) => {
    try {
      await calendarService.deleteEvent(id);
      refreshEvents();
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <div className="flex h-full w-full bg-[#121212] overflow-hidden text-gray-200">
      <CalendarSidebar 
        currentDate={currentDate} 
        setCurrentDate={setCurrentDate} 
        onNewEvent={() => { setIsCreateModalOpen(true); setEditingEvent(null); }}
      />
      <div className="flex-1 min-w-0 flex flex-col border-l border-white/5 bg-[#181818]">
        <CalendarContent 
          currentDate={currentDate} 
          setCurrentDate={setCurrentDate} 
          events={events}
          onDeleteEvent={handleDeleteEvent}
          onEditEvent={handleEditEvent}
        />
      </div>

      <CreateEventModal 
        isOpen={isCreateModalOpen} 
        onClose={() => { setIsCreateModalOpen(false); setEditingEvent(null); }} 
        onCreate={handleSaveEvent}
        selectedDate={currentDate}
        initialEvent={editingEvent}
      />
    </div>
  );
};
