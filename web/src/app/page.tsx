import { RealTimeMetrics } from '@/components/real-time-metrics';

export default function Home() {
  return (
    <div className="flex items-center justify-center min-h-screen p-16">
      <div className="bg-white shadow-md rounded-lg p-8">
        <div className="p-4">
          <RealTimeMetrics />
        </div>
      </div>
    </div>
  );
}
