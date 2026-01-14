import { invoke } from "@tauri-apps/api/core";
import { useCallback } from "react";
import type { ModelSearchResult } from "./use-renderware-state";

export function useModelSearch() {
  const searchSampModels = useCallback(
    async (
      query: string,
      setModelSearchResults: (results: ModelSearchResult[]) => void,
      setIsSearchingModels: (searching: boolean) => void
    ) => {
      if (!query.trim()) {
        setModelSearchResults([]);
        return;
      }

      setIsSearchingModels(true);
      try {
        const results = (await invoke("search_samp_models_by_name", {
          query: query.trim(),
          limit: 20,
        })) as ModelSearchResult[];

        setModelSearchResults(results);
      } catch (error) {
        console.error("Error searching SA:MP models:", error);
        setModelSearchResults([]);
      } finally {
        setIsSearchingModels(false);
      }
    },
    []
  );

  const handleModelSearchInput = useCallback(
    (
      value: string,
      setModelSearchQuery: (query: string) => void,
      searchSampModels: (
        query: string,
        setModelSearchResults: (results: ModelSearchResult[]) => void,
        setIsSearchingModels: (searching: boolean) => void
      ) => void,
      setModelSearchResults: (results: ModelSearchResult[]) => void,
      setIsSearchingModels: (searching: boolean) => void
    ) => {
      setModelSearchQuery(value);
      const timeoutId = setTimeout(() => {
        searchSampModels(value, setModelSearchResults, setIsSearchingModels);
      }, 300);

      return () => clearTimeout(timeoutId);
    },
    []
  );

  return {
    searchSampModels,
    handleModelSearchInput,
  };
}
