import "twin.macro";

export const CardTailWindExample = () => {
  return (
    <div tw="inset-0 flex items-end justify-center pointer-events-none sm:p-6 sm:items-start sm:justify-end">
      <div tw="max-w-sm w-full bg-white shadow-lg rounded-lg pointer-events-auto">
        <div tw="rounded-lg overflow-hidden">
          <div tw="p-4">
            <div tw="flex items-start">
              <div tw="flex-shrink-0">
                <svg
                  tw="h-6 w-6 text-green-400"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
              </div>
              <div tw="ml-3 w-0 flex-1 pt-0.5">
                <p tw="text-sm font-medium text-gray-900">Successfully saved!</p>
                <p tw="mt-1 text-sm text-gray-500">Anyone with a link can now view this file.</p>
              </div>
              <div tw="ml-4 flex-shrink-0 flex">
                <button tw="inline-flex text-gray-400 focus:outline-none focus:text-gray-500 transition ease-in-out duration-150">
                  <svg tw="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                    <path
                      fillRule="evenodd"
                      d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                      clipRule="evenodd"
                    />
                  </svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
