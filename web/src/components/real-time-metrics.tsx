'use client';

import { useEffect, useState } from 'react';
import { Line } from 'react-chartjs-2';
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler
);

export function RealTimeMetrics() {
  const [metrics, setMetrics] = useState({
    interface: 'N/A',
    current: {
      timestamp: new Date(),
      bytes_received: 0,
      bytes_sent: 0,
      bytes_per_second_in: 0,
      bytes_per_second_out: 0,
      transfer_rate_in: '0 KB/s',
      transfer_rate_out: '0 KB/s',
    },
    recent: [],
  });

  const MAX_DATA_POINTS = 20;

  const [chartData, setChartData] = useState<{
    labels: string[];
    datasets: {
      label: string;
      data: number[];
      borderColor: string;
      backgroundColor: string;
      fill: boolean;
      tension: number;
    }[];
  }>({
    labels: [],
    datasets: [
      {
        label: 'Transfer Rate In (KB/s)',
        data: [],
        borderColor: 'rgb(9, 76, 220)',
        backgroundColor: 'rgba(13, 117, 201, 0.2)',
        fill: true,
        tension: 0.4,
      },
      {
        label: 'Transfer Rate Out (KB/s)',
        data: [],
        borderColor: 'rgba(255, 99, 132, 1)',
        backgroundColor: 'rgba(255, 99, 132, 0.2)',
        fill: true,
        tension: 0.4,
      },
    ],
  });

  useEffect(() => {
    const ws = new WebSocket('ws://localhost:8000/ws');

    ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      setMetrics(data);

      setChartData((prev) => {
        const newLabels = [...prev.labels, new Date(data.current.timestamp).toLocaleTimeString()];
        const newBytesIn = [...prev.datasets[0].data, data.current.bytes_per_second_in / 1024];
        const newBytesOut = [...prev.datasets[1].data, data.current.bytes_per_second_out / 1024];

        return {
          labels: newLabels.slice(-MAX_DATA_POINTS),
          datasets: [
            {
              ...prev.datasets[0],
              data: newBytesIn.slice(-MAX_DATA_POINTS),
            },
            {
              ...prev.datasets[1],
              data: newBytesOut.slice(-MAX_DATA_POINTS),
            },
          ],
        };
      });
    };

    ws.onclose = () => {
      console.log('WebSocket connection closed');
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
    };

    return () => {
      ws.close();
    };
  }, []);

  return (
    <div className="w-full max-w-4xl mx-auto p-2 sm:p-4 bg-white rounded-lg">
      <h2 className="text-xl sm:text-2xl font-bold text-center mb-4 sm:mb-6 text-gray-800 dark:text-white">Real-Time Network Metrics</h2>

      {/* Metrics Cards */}
      <div className="grid grid-cols-2 sm:grid-cols-2 lg:grid-cols-4 gap-2 sm:gap-4 mb-4 sm:mb-6">
        <div className="bg-gray-50 dark:bg-gray-700 p-3 sm:p-4 rounded-lg shadow-sm">
          <p className="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Interface</p>
          <p className="text-base sm:text-lg font-semibold truncate">{metrics.interface}</p>
        </div>
        <div className="bg-gray-50 dark:bg-gray-700 p-3 sm:p-4 rounded-lg shadow-sm">
          <p className="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Bytes Received</p>
          <p className="text-base sm:text-lg font-semibold truncate">{metrics.current.bytes_received.toLocaleString()}</p>
        </div>
        <div className="bg-gray-50 dark:bg-gray-700 p-3 sm:p-4 rounded-lg shadow-sm">
          <p className="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Bytes Sent</p>
          <p className="text-base sm:text-lg font-semibold truncate">{metrics.current.bytes_sent.toLocaleString()}</p>
        </div>
        <div className="bg-gray-50 dark:bg-gray-700 p-3 sm:p-4 rounded-lg shadow-sm">
          <p className="text-xs sm:text-sm text-gray-500 dark:text-gray-400">Last Interface Refresh</p>
          <p className="text-base sm:text-lg font-semibold truncate">{new Date(metrics.current.timestamp).toLocaleTimeString()}</p>
        </div>
      </div>

      {/* Current Transfer Rates */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 sm:gap-4 mb-4 sm:mb-8">
        <div className="bg-blue-50 dark:bg-blue-900/20 p-3 sm:p-4 rounded-lg shadow-sm border border-blue-100 dark:border-blue-800">
          <p className="text-xs sm:text-sm text-blue-600 dark:text-blue-300">Transfer Rate In</p>
          <p className="text-xl sm:text-2xl font-bold text-blue-700 dark:text-blue-200">{metrics.current.transfer_rate_in}</p>
        </div>
        <div className="bg-red-50 dark:bg-red-900/20 p-3 sm:p-4 rounded-lg shadow-sm border border-red-100 dark:border-red-800">
          <p className="text-xs sm:text-sm text-red-600 dark:text-red-300">Transfer Rate Out</p>
          <p className="text-xl sm:text-2xl font-bold text-red-700 dark:text-red-200">{metrics.current.transfer_rate_out}</p>
        </div>
      </div>

      {/* Chart */}
      <div className="bg-gray-50 dark:bg-gray-700 p-2 sm:p-4 rounded-lg shadow-sm h-60 sm:h-80">
        <Line
          data={chartData}
          options={{
            responsive: true,
            maintainAspectRatio: false,
            scales: {
              y: {
                beginAtZero: true,
                title: {
                  display: true,
                  text: 'KB/s',
                  color: '#6B7280'
                },
                grid: {
                  color: 'rgba(107, 114, 128, 0.1)'
                },
                ticks: {
                  font: {
                    size: window.innerWidth < 640 ? 8 : 10
                  }
                }
              },
              x: {
                ticks: {
                  maxRotation: 45,
                  minRotation: 45,
                  font: {
                    size: window.innerWidth < 640 ? 8 : 10
                  },
                  autoSkip: true,
                  maxTicksLimit: window.innerWidth < 640 ? 8 : 20
                },
                grid: {
                  display: false
                }
              }
            },
            plugins: {
              legend: {
                position: 'top',
                labels: {
                  usePointStyle: true,
                  boxWidth: 6,
                  font: {
                    size: window.innerWidth < 640 ? 10 : 12
                  }
                }
              },
              title: {
                display: true,
                text: 'Network Traffic History',
                font: {
                  size: window.innerWidth < 640 ? 14 : 16,
                  weight: 'bold'
                }
              },
              tooltip: {
                mode: 'index',
                intersect: false,
                backgroundColor: 'rgba(0, 0, 0, 0.7)',
                padding: 8,
                cornerRadius: 4,
                bodyFont: {
                  size: window.innerWidth < 640 ? 10 : 12
                },
                titleFont: {
                  size: window.innerWidth < 640 ? 10 : 12
                }
              }
            },
            interaction: {
              mode: 'nearest',
              axis: 'x',
              intersect: false
            },
            animation: {
              duration: 0
            }
          }}
        />
      </div>
    </div>
  );
}
