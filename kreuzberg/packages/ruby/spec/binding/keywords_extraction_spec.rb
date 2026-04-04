# frozen_string_literal: true

RSpec.describe 'Keyword Extraction' do
  describe 'integration with Extraction config' do
    it 'accepts Keywords config in Extraction' do
      keywords = Kreuzberg::Config::Keywords.new(
        algorithm: 'yake',
        max_keywords: 10
      )
      config = Kreuzberg::Config::Extraction.new(keywords: keywords)

      expect(config.keywords).to be_a(Kreuzberg::Config::Keywords)
      expect(config.keywords.algorithm).to eq('yake')
    end

    it 'accepts keywords config as hash in Extraction' do
      config = Kreuzberg::Config::Extraction.new(
        keywords: {
          algorithm: 'rake',
          max_keywords: 15,
          min_score: 0.3
        }
      )

      expect(config.keywords).to be_a(Kreuzberg::Config::Keywords)
      expect(config.keywords.algorithm).to eq('rake')
      expect(config.keywords.max_keywords).to eq(15)
    end

    it 'includes keywords config in to_h' do
      keywords = Kreuzberg::Config::Keywords.new(
        algorithm: 'yake',
        max_keywords: 10
      )
      config = Kreuzberg::Config::Extraction.new(keywords: keywords)

      hash = config.to_h

      expect(hash).to include(:keywords)
      expect(hash[:keywords]).to be_a(Hash)
      expect(hash[:keywords][:algorithm]).to eq('yake')
    end

    it 'handles nil keywords config' do
      config = Kreuzberg::Config::Extraction.new(keywords: nil)

      expect(config.keywords).to be_nil
      hash = config.to_h
      expect(hash[:keywords]).to be_nil
    end
  end
end
